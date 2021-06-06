use crate::interrupt_controller;
use crate::register_state::GeneralPurposeRegisterState;
use crate::vmcs_dump;
use crate::vmcs_fields::VmcsField;
use crate::vmexit_reasons::*;
use crate::vmx::vmread;
use crate::vmx::vmwrite;
use log::trace;

fn advance_guest_rip() -> Result<(), x86::vmx::VmFail> {
    let mut rip = vmread(VmcsField::GuestRip)?;
    let len = vmread(VmcsField::VmExitInstructionLen)?;
    rip += len;
    vmwrite(VmcsField::GuestRip, rip)
}

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

#[no_mangle]
pub extern "C" fn hypervisor_handle_vmexit(gprs: *mut GeneralPurposeRegisterState) {
    let gprs = unsafe { &mut *gprs };
    let vmexit_reasion = vmread(VmcsField::VmExitReason).expect("vm exit reason shouldn't error");
    let qualification = vmread(VmcsField::ExitQualificatIon).unwrap_or(0);
    match vmexit_reasion {
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

#[no_mangle]
pub extern "C" fn hypervisor_vmresume_failure() {
    unimplemented!();
}
