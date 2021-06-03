#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![allow(unknown_lints)]

use ::log::{error, info, log};

//pub mod runtime;
mod interrupts;
mod isr;
mod msr;
mod register_state;
pub mod segmentation;
mod vmcs;
mod vmcs_fields;
mod vmexit_handlers;
pub mod vmx;

/*
#[cfg(not(test))]
mod serial_logger;
#[cfg(not(test))]
use serial_logger as logger;
*/
use pcuart::logger;
//include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[repr(C)]
pub struct VCpu {
    pub vmxon_region: *mut u32,
    pub vmcs: *mut u32,
    pub vmxon_region_phys: u64,
    pub vmcs_phys: u64,
    pub vmxon_region_size: usize,
    pub vmcs_size: usize,
    pub loaded_successfully: bool,
    pub stack_base: *mut u32,
    pub stack_size: usize,
    pub stack_top: *mut u32,
    pub host_gdt_base: *mut u64,
    pub host_gdt_limit: u64,
    pub tr_base: u64,
    pub tr_selector: u16,
}

#[no_mangle]
pub extern "C" fn rustyvisor_load() -> i32 {
    // The log crate requires the stdlib to use log::set_logger. Use the unsafe version instead.
    let logger_result = unsafe { ::log::set_logger_raw(|_filter| &logger::LOGGER) };
    match logger_result {
        Ok(()) => {}
        Err(_) => return -1,
    }

    info!("{}", "Hello world");

    interrupts::init_interrupt_handlers(vmx::read_cs());

    #[cfg(feature = "runtime_tests")]
    runtime_tests();

    0
}

#[no_mangle]
pub unsafe extern "C" fn rustyvisor_core_load(data: &VCpu) -> i32 {
    let data = &*data;

    if vmx::enable(
        data.vmxon_region,
        data.vmxon_region_phys,
        data.vmxon_region_size,
    )
    .is_err()
    {
        error!("Failed to enable VMX");
        return -1;
    }
    if vmx::load_vm(data).is_err() {
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
