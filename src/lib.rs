#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![allow(unknown_lints)]

use ::log::{error, info, log};

pub mod runtime;
pub mod vmx;

#[cfg(not(test))]
mod serial_logger;
#[cfg(not(test))]
use serial_logger as logger;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[repr(C)]
pub struct PerCoreData {
    vmxon_region: *mut u8,
    vmcs: *mut u8,
    vmxon_region_phys: u64,
    vmcs_phys: u64,
    vmxon_region_size: usize,
    vmcs_size: usize,
    loaded_successfully: bool,
}

#[no_mangle]
pub extern "C" fn rustyvisor_load() -> i32 {
    #[cfg(not(test))]
    {
        match logger::init() {
            Ok(()) => {}
            Err(_) => return 1,
        }
    }

    info!("{}", VERSION);

    #[cfg(feature = "runtime_tests")]
    runtime_tests();

    0
}

#[no_mangle]
pub unsafe extern "C" fn rustyvisor_core_load(data: *const PerCoreData) -> i32 {
    if data.is_null() {
        return 1;
    }

    let data = &*data;

    if vmx::enable(
        data.vmxon_region,
        data.vmxon_region_phys,
        data.vmxon_region_size,
    )
    .is_err()
    {
        error!("Failed to enable VMX");
        return 1;
    }

    if vmx::load_vm(data.vmcs, data.vmcs_phys, data.vmcs_size).is_err() {
        error!("Failed to load VMX");
        return 1;
    }

    info!("Successfully launched VM");
    0
}

#[no_mangle]
pub extern "C" fn rustyvisor_core_unload() {
    info!("Core unload");
    vmx::unload_vm();
    vmx::disable();
}

#[no_mangle]
pub extern "C" fn rustyvisor_unload() {
    info!("Hypervisor unloaded.");

    #[cfg(not(test))]
    {
        let _ = logger::fini();
    }
}

#[cfg(feature = "runtime_tests")]
fn runtime_tests() {
    info!("Executing runtime tests...");
    info!("Runtime tests succeeded");
}
