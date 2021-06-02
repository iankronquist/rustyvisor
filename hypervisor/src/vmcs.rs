use core::convert::TryFrom;
use crate::vmx::{vmwrite, read_cr0,read_cr3, read_cr4,read_cs,read_db7, read_ds, read_es,read_fs,read_gs,read_ss};
use crate::msr::{Msr, rdmsr};
use crate::vmcs_fields::*;
use crate::segmentation;
use crate::PerCoreData;

extern "C" {
    fn _host_entrypoint();
}

pub fn initialize_host_state(vcpu: &PerCoreData) -> Result<(), u32>{
    vmwrite(VmcsField::HostCr0, read_cr0())?;
    vmwrite(VmcsField::HostCr3, read_cr3())?;
    vmwrite(VmcsField::HostCr4, read_cr4())?;

    let cs = read_cs();
    vmwrite(VmcsField::HostCsSelector, u64::from(cs))?;
    let ds = read_ds();
    vmwrite(VmcsField::HostDsSelector, u64::from(ds))?;
    let es = read_es();
    vmwrite(VmcsField::HostEsSelector, u64::from(es))?;
    let fs = read_fs();
    vmwrite(VmcsField::HostFsSelector, u64::from(fs))?;
    let gs = read_gs();
    vmwrite(VmcsField::HostGsSelector, u64::from(gs))?;
    let ss = read_ss();
    vmwrite(VmcsField::HostSsSelector, u64::from(ss))?;
    vmwrite(VmcsField::HostTrSelector, u64::from(vcpu.tr_selector))?;
    vmwrite(VmcsField::HostTrBase, vcpu.tr_base)?;

    vmwrite(VmcsField::HostGdtrBase, vcpu.host_gdt_base as u64)?;
    vmwrite(VmcsField::HostIdtrBase, crate::interrupts::host_idt_base())?;
    vmwrite(VmcsField::HostFsBase, vcpu as *const PerCoreData as u64)?;
    vmwrite(VmcsField::HostGsBase, 0)?;


    vmwrite(VmcsField::HostRsp, vcpu.stack_top as u64)?;
    vmwrite(VmcsField::HostRip, _host_entrypoint as u64)?;

    Ok(())
}

pub fn initialize_guest_state() {}

pub fn adjust_value_based_on_msr(msr: Msr, controls: u64) -> u64{
    let controls = u32::try_from(controls).expect("Controls should be a 32 bit field"); // 503 953 2390
    let (fixed0, fixed1) = rdmsr(msr);
    assert_eq!(controls & fixed0, controls);
    u64::from(fixed1 | (controls & fixed0))
}

pub fn initialize_vm_control_values() -> Result<(), u32> {
    
    // Configure entry/exit and supported feature controls
    vmwrite(VmcsField::SecondaryVmExecControl,
        adjust_value_based_on_msr(Msr::Ia32VmxProcBasedControls2,
            SecondaryCpuBasedControlsRdtscpEnable | SecondaryCpuBasedControlsInvpcidEnable | SecondaryCpuBasedControlsXSavesEnable))?;

    vmwrite(VmcsField::PinBasedVmExecControl,
        adjust_value_based_on_msr(Msr::Ia32VmxPinBasedControls, PinBasedControlsVmxPreemption | PinBasedControlsExternalInterruptExiting))?;

    vmwrite(VmcsField::VmxPreemptionTimerValue, 0xfffff)?;

    vmwrite(VmcsField::CpuBasedVmExecControl,
        adjust_value_based_on_msr(Msr::Ia32VmxProcBasedControls,
            CpuBasedControlsMsrBitmaps | CpuBasedControlsSecondaryEnable | CpuBasedControlsIoBitmaps | CpuBasedControlsIoExiting))?;

    vmwrite(VmcsField::VmExitControls,
        adjust_value_based_on_msr(Msr::Ia32VmxExitControls,
            VmExitIa32eMode | VmExitAcknowledgeInterruptOnExit | VmExitConcealVmxFromPt))?;

    vmwrite(VmcsField::VmEntryControls,
        adjust_value_based_on_msr(Msr::Ia32VmxEntryControls,
            VmEntryIa32eMode))?;
    Ok(())
}


pub fn vm_instruction_error_number_message(n: u64) -> &'static str {
    match n {
        1 => "VMALL executed in VMX root operation",
        2 => "VMCLEAR with invalid physical address",
        3 => "VMCLEAR with VMXON pointer",
        4 => "VMLAUNCH with non-clear VMCS",
        5 => "VMRESUME with non-launched VMCS",
        6 => "VMRESUME after VMXOFF (VMXOFF and VMXON between VMLAUNCH and VMRESUME)",
        7 => "VM entry with invalid control field(s)",
        8 => "VM entry with invalid host-state field(s)",
        9 => "VMPTRLD with invalid physical address",
        10 => "VMPTRLD with VMXON pointer",
        11 => "VMPTRLD with incorrect VMCS revision identifier",
        12 => "VMREAD/VMWRITE from/to unsupported VMCS component",
        13 => "VMWRITE to read-only VMCS component",
        15 => "VMXON executed in VMX root operation",
        16 => "VM entry with invalid executive-VMCS pointer",
        17 => "VM entry with non-launched executive VMCS",
        18 => {
            "VM entry with executive-VMCS pointer not VMXON pointer (when attempting to deactivate \
             the dual-monitor treatment of"
        }
        19 => {
            "VMCALL with non-clear VMCS (when attempting to activate the dual-monitor treatment of \
             SMIs and SMM)"
        }
        20 => "VMCALL with invalid VM-exit control fields",
        22 => {
            "VMCALL with incorrect MSEG revision identifier (when attempting to activate the \
             dual-monitor treatment of SMIs
            and SMM)"
        }
        23 => "VMXOFF under dual-monitor treatment of SMIs and SMM",
        24 => {
            "VMCALL with invalid SMM-monitor features (when attempting to activate the \
             dual-monitor treatment of SMIs and SMM)"
        }
        25 => {
            "VM entry with invalid VM-execution control fields in executive VMCS (when attempting \
             to return from SMM)"
        }
        26 => "VM entry with events blocked by MOV SS.",
        28 => "Invalid operand to INVEPT/INVVPID.",
        _ => "Unknown VM instruction error number.",
    }
}