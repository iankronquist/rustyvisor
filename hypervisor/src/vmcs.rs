use crate::msr::{rdmsr, rdmsrl, Msr};
use crate::segmentation::{get_current_gdt, unpack_gdt_entry};
use crate::vmcs_fields::*;
use crate::vmx::{ read_dr7, vmread, vmwrite, };
use crate::VCpu;
use core::convert::TryFrom;
use log::{trace, warn};
use x86::dtables;

extern "C" {
    fn _host_entrypoint();
}

pub fn initialize_host_state(vcpu: &VCpu) -> Result<(), x86::vmx::VmFail> {

    let cr0 = unsafe { x86::controlregs::cr0() }.bits() as u64;
    let cr3 = unsafe { x86::controlregs::cr3() };
    let cr4 = unsafe { x86::controlregs::cr4() }.bits() as u64;
    vmwrite(VmcsField::HostCr0, cr0)?;
    vmwrite(VmcsField::HostCr3, cr3)?;
    vmwrite(VmcsField::HostCr4, cr4)?;

    let cs = x86::segmentation::cs();
    vmwrite(VmcsField::HostCsSelector, u64::from(cs.bits()))?;
    let ds = x86::segmentation::ds();
    vmwrite(VmcsField::HostDsSelector, u64::from(ds.bits()))?;
    let es = x86::segmentation::es();
    vmwrite(VmcsField::HostEsSelector, u64::from(es.bits()))?;
    let fs = x86::segmentation::fs();
    vmwrite(VmcsField::HostFsSelector, u64::from(fs.bits()))?;
    let gs = x86::segmentation::gs();
    vmwrite(VmcsField::HostGsSelector, u64::from(gs.bits()))?;
    let ss = x86::segmentation::ss();
    vmwrite(VmcsField::HostSsSelector, u64::from(ss.bits()))?;
    assert_eq!(vcpu.tr_selector & !0x7, vcpu.tr_selector); // TR RPL must be 0. See host entry error reasons chapter.
    vmwrite(VmcsField::HostTrSelector, u64::from(vcpu.tr_selector))?;
    vmwrite(VmcsField::HostTrBase, vcpu.tr_base)?;

    vmwrite(VmcsField::HostGdtrBase, vcpu.host_gdt_base as u64)?;
    vmwrite(VmcsField::HostIdtrBase, crate::interrupts::host_idt_base())?;
    vmwrite(VmcsField::HostFsBase, vcpu as *const VCpu as u64)?;
    vmwrite(VmcsField::HostGsBase, 0)?;

    vmwrite(VmcsField::HostRsp, vcpu.stack_top as u64)?;
    vmwrite(VmcsField::HostRip, _host_entrypoint as usize as u64)?;

    Ok(())
}

pub fn initialize_guest_state(_vcpu: &VCpu) -> Result<(), x86::vmx::VmFail> {
    trace!("initialize_guest_state");
    vmwrite(VmcsField::VmcsLinkPointer, !0)?;

    let mut guest_idtr: dtables::DescriptorTablePointer<u64> = Default::default();
    unsafe {
        dtables::sidt(&mut guest_idtr);
    }
    trace!("got idtr {:x?}", guest_idtr);
    vmwrite(VmcsField::GuestIdtrLimit, u64::from(guest_idtr.limit))?;
    vmwrite(VmcsField::GuestIdtrBase, guest_idtr.base as u64)?;

    let mut guest_gdtr: dtables::DescriptorTablePointer<u64> = Default::default();
    unsafe {
        dtables::sgdt(&mut guest_gdtr);
    }
    trace!("got idtr {:x?}", guest_gdtr);
    vmwrite(VmcsField::GuestGdtrLimit, u64::from(guest_gdtr.limit))?;
    vmwrite(VmcsField::GuestGdtrBase, guest_gdtr.base as u64)?;

    let gdt = get_current_gdt();
    trace!("got gdt {:x?}", gdt);

    let cs = x86::segmentation::cs();
    let cs_unpacked = unpack_gdt_entry(gdt, cs.bits());
    vmwrite(VmcsField::GuestCsSelector, u64::from(cs_unpacked.selector))?;
    vmwrite(VmcsField::GuestCsLimit, cs_unpacked.limit)?;
    vmwrite(
        VmcsField::GuestCsArBytes,
        u64::from(cs_unpacked.access_rights),
    )?;
    vmwrite(VmcsField::GuestCsBase, cs_unpacked.base)?;

    let ds = x86::segmentation::ds();
    let ds_unpacked = unpack_gdt_entry(gdt, ds.bits());
    vmwrite(VmcsField::GuestDsSelector, u64::from(ds_unpacked.selector))?;
    vmwrite(VmcsField::GuestDsLimit, ds_unpacked.limit)?;
    vmwrite(
        VmcsField::GuestDsArBytes,
        u64::from(ds_unpacked.access_rights),
    )?;
    vmwrite(VmcsField::GuestDsBase, ds_unpacked.base)?;

    let es = x86::segmentation::es();
    let es_unpacked = unpack_gdt_entry(gdt, es.bits());
    vmwrite(VmcsField::GuestEsSelector, u64::from(es_unpacked.selector))?;
    vmwrite(VmcsField::GuestEsLimit, es_unpacked.limit)?;
    vmwrite(
        VmcsField::GuestEsArBytes,
        u64::from(es_unpacked.access_rights),
    )?;
    vmwrite(VmcsField::GuestEsBase, es_unpacked.base)?;

    let fs = x86::segmentation::fs();
    let fs_unpacked = unpack_gdt_entry(gdt, fs.bits());
    vmwrite(VmcsField::GuestFsSelector, u64::from(fs_unpacked.selector))?;
    vmwrite(VmcsField::GuestFsLimit, fs_unpacked.limit)?;
    vmwrite(
        VmcsField::GuestFsArBytes,
        u64::from(fs_unpacked.access_rights),
    )?;
    vmwrite(VmcsField::GuestFsBase, fs_unpacked.base)?;

    let gs = x86::segmentation::gs();
    let gs_unpacked = unpack_gdt_entry(gdt, gs.bits());
    vmwrite(VmcsField::GuestGsSelector, u64::from(gs_unpacked.selector))?;
    vmwrite(VmcsField::GuestGsLimit, gs_unpacked.limit)?;
    vmwrite(
        VmcsField::GuestGsArBytes,
        u64::from(gs_unpacked.access_rights),
    )?;
    vmwrite(VmcsField::GuestGsBase, gs_unpacked.base)?;

    let ss = x86::segmentation::ss();
    let ss_unpacked = unpack_gdt_entry(gdt, ss.bits());
    vmwrite(VmcsField::GuestSsSelector, u64::from(ss_unpacked.selector))?;
    vmwrite(VmcsField::GuestSsLimit, ss_unpacked.limit)?;
    vmwrite(
        VmcsField::GuestSsArBytes,
        u64::from(ss_unpacked.access_rights),
    )?;
    vmwrite(VmcsField::GuestSsBase, ss_unpacked.base)?;

    let tr = x86::task::tr();
    let tr_unpacked = unpack_gdt_entry(gdt, tr.bits());
    vmwrite(VmcsField::GuestTrSelector, u64::from(tr_unpacked.selector))?;
    vmwrite(VmcsField::GuestTrLimit, tr_unpacked.limit)?;
    if tr_unpacked.is_usable() {
        vmwrite(
            VmcsField::GuestTrArBytes,
            u64::from(tr_unpacked.access_rights),
        )?;
    } else {
        // 26.3.1.2     Checks on Guest Segment Registers
        // Vol. 3C   26-11
        // Set present (bit 7), 64 bit (0xb in 0:3), rest is clear.
        vmwrite(VmcsField::GuestTrArBytes, (1 << 7) | 0xb)?;
    }
    vmwrite(VmcsField::GuestTrBase, tr_unpacked.base)?;

    let ldtr = unsafe { x86::dtables::ldtr() };
    let ldtr_unpacked = unpack_gdt_entry(gdt, ldtr.bits());
    vmwrite(
        VmcsField::GuestLdtrSelector,
        u64::from(ldtr_unpacked.selector),
    )?;
    vmwrite(VmcsField::GuestLdtrLimit, ldtr_unpacked.limit)?;
    vmwrite(
        VmcsField::GuestLdtrArBytes,
        u64::from(ldtr_unpacked.access_rights),
    )?;
    vmwrite(VmcsField::GuestLdtrBase, ldtr_unpacked.base)?;

    let cr4 = unsafe { x86::controlregs::cr4() };
    vmwrite(VmcsField::GuestCr4, cr4.bits() as u64)?;
    //vmwrite(VmcsField::GuestCr4ReadShadow, cr4)?;
    let cr3 = unsafe { x86::controlregs::cr3() };
    vmwrite(VmcsField::GuestCr3, cr3)?;
    let cr0 = unsafe { x86::controlregs::cr0() };
    vmwrite(VmcsField::GuestCr0, cr0.bits() as u64)?;
    //vmwrite(VmcsField::GuestCr0ReadShadow, cr0)?;
    vmwrite(VmcsField::GuestIA32Debugctl, rdmsrl(Msr::Ia32DebugControl))?;
    let dr7 = read_dr7();
    vmwrite(VmcsField::GuestDr7, dr7)?;

    Ok(())
}

pub fn adjust_value_based_on_msr(msr: Msr, controls: u64) -> u64 {
    let controls = u32::try_from(controls).expect("Controls should be a 32 bit field"); // 503 953 2390
    let pair = rdmsr(msr);
    let fixed0 = pair.edx;
    let fixed1 = pair.eax;
    if controls & fixed0 != controls {
        warn!(
            "Requested unsupported controls for msr {:?}, fixed0 {:x} fixed1 {:x} controls {:x}",
            msr, fixed0, fixed1, controls
        );
    }
    u64::from(fixed1 | (controls & fixed0))
}

pub fn initialize_vm_control_values(vcpu: &VCpu) -> Result<(), x86::vmx::VmFail> {
    // Configure entry/exit and supported feature controls
    vmwrite(
        VmcsField::SecondaryVmExecControl,
        adjust_value_based_on_msr(
            Msr::Ia32VmxProcBasedControls2,
            SecondaryCpuBasedControlsRdtscpEnable
                | SecondaryCpuBasedControlsInvpcidEnable
                | SecondaryCpuBasedControlsXSavesEnable,
        ),
    )?;

    vmwrite(
        VmcsField::PinBasedVmExecControl,
        adjust_value_based_on_msr(
            Msr::Ia32VmxPinBasedControls,
            /*PinBasedControlsVmxPreemption | PinBasedControlsExternalInterruptExiting*/ 0,
        ),
    )?;

    let pin = vmread(VmcsField::PinBasedVmExecControl)?;
    if pin & PinBasedControlsExternalInterruptExiting != 0 {
        warn!("External interrupt exiting enabled");
    } else {
        trace!("External interrupt exiting not enabled");
    }

    vmwrite(VmcsField::VmxPreemptionTimerValue, 0xfffff)?;

    vmwrite(
        VmcsField::CpuBasedVmExecControl,
        adjust_value_based_on_msr(
            Msr::Ia32VmxProcBasedControls,
            CpuBasedControlsMsrBitmaps |
                CpuBasedControlsSecondaryEnable
                //| CpuBasedControlsIoBitmaps
                //| CpuBasedControlsIoExiting,
        ),
    )?;

    vmwrite(
        VmcsField::VmExitControls,
        adjust_value_based_on_msr(
            Msr::Ia32VmxExitControls,
            VmExitIa32eMode /*| VmExitAcknowledgeInterruptOnExit*/ | VmExitConcealVmxFromPt,
        ),
    )?;

    vmwrite(
        VmcsField::VmEntryControls,
        adjust_value_based_on_msr(Msr::Ia32VmxEntryControls, VmEntryIa32eMode),
    )?;

    vmwrite(VmcsField::MsrBitmap, vcpu.msr_bitmap as u64)?;

    Ok(())
}

pub fn vm_instruction_error_number_message(n: u64) -> &'static str {
    match n {
        0 => "No error",
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
