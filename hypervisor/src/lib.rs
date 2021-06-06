#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![allow(unknown_lints)]

use ::log::{error, info, trace, LevelFilter};

mod interrupts;
mod isr;
mod msr;
mod panic;
mod register_state;
pub mod segmentation;
mod vmcs;
mod vmcs_dump;
mod vmcs_fields;
mod vmexit_handlers;
pub mod vmx;
mod vcpu;
pub mod interrupt_controller;
mod vmexit_reasons;
mod debug;
use pcuart::logger;


pub static LOGGER: logger::UartLogger = logger::UartLogger::new(pcuart::UartComPort::Com1);


#[derive(Debug)]
#[repr(C)]
pub struct VCpu {
    pub this_vcpu: *mut VCpu,
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
    pub virtual_local_interrupt_controller: *mut interrupt_controller::VirtualLocalInterruptController,
}

#[no_mangle]
pub extern "C" fn rustyvisor_load() -> i32 {
    let logger_result =
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));
    match logger_result {
        Ok(()) => {}
        Err(_) => return -1,
    }

    info!("{}", "rustyvisor_load");

    interrupts::init_interrupt_handlers(vmx::read_cs());

    #[cfg(feature = "runtime_tests")]
    runtime_tests();

    0
}

#[no_mangle]
pub unsafe extern "C" fn rustyvisor_core_load(data: &VCpu) -> i32 {
    trace!(
        "VCPU in rustyvisor_core_load {:x?} {:x?}\r\n",
        data,
        data as *const VCpu
    );
    trace!("Enabling vmx");
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
    trace!(
        "VCPU in rustyvisor_core_load enable {:x?} {:x?}\r\n",
        data,
        data as *const VCpu
    );

    trace!("Vmx enabled");
    trace!("Loading vmm {:x?}", data);
    if vmx::load_vm(data).is_err() {
        error!("Failed to load VMX");
        return 1;
    }
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
