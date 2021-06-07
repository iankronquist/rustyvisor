//! This module defines functions for working with VCpus.
use crate::VCpu;
use x86::bits64::segmentation::fs_deref;

/// Get a reference to the current VCpu structure.
/// The reference is static because the VCpu structure must outlive the
/// hypervisor.
/// Each CPU has data used to represent the guest stored in the VCpu structure.
/// A pointer to the VCpu structure for this core is placed in the fs base
/// register.
/// The first field of the vcpu structure is a pointer to itself, so if we load
/// the first pointer of the fs base region, that is fs:0, we get a pointer to
/// the current VCpu.
pub fn get_current_vcpu() -> &'static mut VCpu {
    unsafe { &mut *(fs_deref() as *mut VCpu) }
}
