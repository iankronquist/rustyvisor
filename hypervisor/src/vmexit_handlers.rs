//! This module defines the host's VM exit handlers.
//use crate::interrupt_controller;
use crate::hypercall_handler;
use crate::register_state::GeneralPurposeRegisterState;
use crate::vmcs_dump;
use crate::vmcs_fields::VmcsField;
use crate::vmexit_reasons::*;
use crate::vmx;
use crate::vmx::vmread;
use crate::vmx::vmwrite;
use crate::VCpu;
use log::trace;

/// Advance the guest's instruction pointer by the length of the instruction
/// being executed by the guest when the VM exit occurred. When the guest
/// resumes, it will start executing the next instruction.
/// This function should not be called more than once per VM exit, or the guest
/// may begin executing illegal or unintended instructions.
fn advance_guest_rip() -> Result<(), x86::vmx::VmFail> {
    let mut rip = vmread(VmcsField::GuestRip)?;
    let len = vmread(VmcsField::VmExitInstructionLen)?;
    rip += len;
    vmwrite(VmcsField::GuestRip, rip)
}

/// Handle CPUID
/// In most cases, report back the host's CPUID values with the following exceptions:
/// - Clear the VMX available bit
/// - If RAX has the magic value 'rsty' or 0x72737479 this is a hypercall, so
///   call the hypercall handler.
/// - Do not set the hypervisor bit, to be stealthy.
fn handle_cpuid(gprs: &mut GeneralPurposeRegisterState) -> Result<(), x86::vmx::VmFail> {
    trace!("cpuid with gprs {:x?}", gprs);
    advance_guest_rip()?;
    if gprs.rax as u32 == hypervisor_abi::HYPERCALL_MAGIC {
        return hypercall_handler::handle_hypercall(gprs);
    }

    let mut result = unsafe { core::arch::x86_64::__cpuid(gprs.rax as u32) };
    if gprs.rax == vmx::CPUIDLeaf::ProcessorInfoAndFeatures as u64 {
        result.ecx &= vmx::CPUIDLeafProcessorInfoAndFeaturesECXBits::VMXAvailable as u32;
    }
    gprs.rax = u64::from(result.eax);
    gprs.rbx = u64::from(result.ebx);
    gprs.rcx = u64::from(result.ecx);
    gprs.rdx = u64::from(result.edx);

    trace!("new gprs {:x?}", gprs);
    Ok(())
}

/// Emulate control register access. When a control register access VM exit
/// occurs, perform that access, e.g. load from the control register or a store
/// to it, on the underlying hardware (since this is a mostly passthrough
/// hypervisor).
/// At present we do not implement the CLTS or LMSW instructions since they are
/// unused by major operating systems.
fn handle_control_register_access(
    gprs: &mut GeneralPurposeRegisterState,
) -> Result<(), x86::vmx::VmFail> {
    // 27-6 vol 3c table 27-3 exit qual for cr access
    let qualification = vmread(VmcsField::ExitQualificatIon)?;

    let crnum = qualification & 0xf;
    let access_type = (qualification >> 4) & 0x3;
    // Presently unimplemented, but here when I need it. See comment below.
    //usize lmsw_type = (qualification >> 6) & 1;
    let regnum = (qualification >> 8) & 0xf;

    let field = match crnum {
        0 => VmcsField::GuestCr0,
        3 => VmcsField::GuestCr3,
        4 => VmcsField::GuestCr4,
        _ => panic!("Illegal crnum from qualification {:x}", qualification),
    };

    let register = gprs.by_mod_rm_index(regnum);

    match access_type {
        // Write
        0 => {
            let value = match register {
                Some(reg) => *reg,
                None => vmread(VmcsField::GuestRsp)?,
            };
            vmwrite(field, value)?;
        }
        // Read
        1 => {
            let value = vmread(field)?;
            match register {
                Some(reg) => {
                    *reg = value;
                }
                None => vmwrite(VmcsField::GuestRsp, value)?,
            }
        }
        // FIXME: implement LMSW & CLTS.
        // I don't believe any major OS uses them.
        _ => {
            unimplemented!("Unhandled CR access. Qualification {:x}", qualification);
        }
    }
    advance_guest_rip()?;
    Ok(())
}

/// Handle a VM Exit. This function will be called by the assembly code in
/// the function _host_entrypoint when a VM exit occurs.
/// This function must handle the exit reason or panic.
/// The guest will be resumed using the current vmcs and the guest general
/// purpose register state when this function returns.
#[no_mangle]
pub extern "C" fn hypervisor_handle_vmexit(_vcpu: *mut VCpu) {
    let vcpu = crate::vcpu::get_current_vcpu();
    //trace!("vcpu fs {:x?}", vcpu_fs);
    let gprs = unsafe { &mut (*vcpu).general_purpose_registers };
    let vmexit_reasion = vmread(VmcsField::VmExitReason).expect("vm exit reason shouldn't error");
    let qualification = vmread(VmcsField::ExitQualificatIon).unwrap_or(0);
    match vmexit_reasion {
        VMEXIT_REASON_CPUID => handle_cpuid(gprs).unwrap(),
        VMEXIT_REASON_CONTROL_REGISTER_ACCESS => {
            handle_control_register_access(gprs).unwrap();
        }
        /*
        VMEXIT_REASON_EXTERNAL_INTERRUPT => {
            trace!("Got external interrupt {:x?}", vmread(VmcsField::GuestRip));
            crate::debug::breakpoint();
            interrupt_controller::received_external_interrupt().unwrap();
            vmcs_dump::dump();
        },
        VMEXIT_REASON_PREEMPTION_TIMER_EXPIRED => {
            trace!("vmx preemption timer expired");
            interrupt_controller::received_preemption_timer().unwrap();
        },
        VMEXIT_REASON_INTERRUPT_WINDOW => {
            trace!("vmx interrupt window available");
            interrupt_controller::received_interrupt_window_exit().unwrap();
        }
        */
        reason => {
            trace!("{:x?}", gprs);
            vmcs_dump::dump();
            panic!(
                "Unhandled vm exit reason {:x} qualification {:x}",
                reason, qualification
            );
        }
    }
}

/// Called by [_host_entrypoint](../vmcs/fn._host_entrypoint.html) when a VM
/// resume failure occurs. If the host cannot resume the guest, it must have
/// misconfigured the guest's state, which is a bug in the hypervisor.
/// This function panics.
#[no_mangle]
pub extern "C" fn hypervisor_vmresume_failure() {
    unimplemented!();
}
