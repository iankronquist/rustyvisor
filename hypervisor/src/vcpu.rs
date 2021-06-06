use crate::VCpu;
use x86::bits64::segmentation::fs_deref;

pub fn get_current_vcpu() -> &'static mut VCpu {
    unsafe { &mut *(fs_deref() as *mut VCpu) }
}
