use log::trace;
use crate::vmcs_dump;
use crate::vmcs_fields::VmcsField;
use crate::vmx::{vmwrite,vmread};
use crate::register_state::GeneralPurposeRegisterState;

#[no_mangle]
pub extern "C" fn hypervisor_handle_vmexit(gprs: *mut GeneralPurposeRegisterState) {
    let gprs = unsafe {&*gprs};
    trace!("{:x?}", gprs);
    let vmexit_reasion = vmread(VmcsField::VmExitReason).expect("vm exit reason shouldn't error");
    let qualification = vmread(VmcsField::ExitQualificatIon).unwrap_or(0);
    trace!("vmexit_reasion {:x}", vmexit_reasion);
    trace!("qualification {:x}", qualification);
    match vmexit_reasion {
        reason => {
            //vmcs_dump::dump();
            panic!("Unhandled vm exit reason {:x} qualification {:x}", vmexit_reasion, qualification);
        }
    }
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn hypervisor_vmresume_failure() {
    unimplemented!();
}