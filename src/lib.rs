#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(use_extern_macros)]
#![feature(integer_atomics)]
#![feature(lang_items)]

#![allow(unknown_lints)]

pub mod cpu;
pub mod vmx;
/*
pub mod cli;
pub mod interrupts;
mod isr;
pub mod runtime;
pub mod segmentation;
*/

pub mod runtime;
#[macro_use]
mod linux;

#[macro_use]
extern crate log;

extern crate spin;

#[cfg(not(test))]
#[cfg(feature = "dmesg_logger")]
#[macro_use]
mod dmesg_logger;
#[cfg(not(test))]
#[cfg(feature = "dmesg_logger")]
use dmesg_logger as logger;

#[cfg(not(test))]
#[cfg(not(feature = "dmesg_logger"))]
mod serial_logger;
#[cfg(not(test))]
#[cfg(not(feature = "dmesg_logger"))]
use serial_logger as logger;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

/*
#[no_mangle]
pub extern "C" fn dispatch_interrupt() {}
*/

#[repr(C)]
pub struct PerCoreData {
	task: *const u8,
	vmxon_region: *mut u8,
	vmcs: *mut u8,
	vmxon_region_phys: u64,
	vmcs_phys: u64,
	vmxon_region_size: usize,
	vmcs_region_size: usize,
	loaded_successfully: bool,
}

#[no_mangle]
pub extern "C" fn rustyvisor_load() -> i32 {
    #[cfg(not(test))]
    {
        match logger::init() {
            Ok(()) => {},
            Err(_) => return 1,
        }
    }

    info!("{}", VERSION);

    #[cfg(feature = "runtime_tests")]
    runtime_tests();

    0

}

#[no_mangle]
pub extern "C" fn rustyvisor_core_load(data: *const PerCoreData) -> i32 {
    error!("core load");
    if data.is_null() {
        return 1;
    }

    unsafe {
        if vmx::enable((*data).vmxon_region, (*data).vmxon_region_phys, (*data).vmxon_region_size) != Ok(()) {
            return 1;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn rustyvisor_core_unload() {
    error!("core unload");
    vmx::disable();
}


#[no_mangle]
pub extern "C" fn rustyvisor_unload() {

    #[cfg(not(test))]
    {
        let _ = logger::fini();
    }

    info!("Hypervisor unloaded.");
}

#[cfg(feature = "runtime_tests")]
fn runtime_tests() {
    info!("Executing runtime tests...");
    info!("Runtime tests succeeded");
}
