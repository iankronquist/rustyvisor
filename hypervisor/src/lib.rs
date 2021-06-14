#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![allow(unknown_lints)]

//! A library implementing a mostly-passthrough hypervisor.
//! A mostly passthrough hypervisor mostly virtualizes the guest and does very
//! little emulation of hardware or devices, but can be used to inspect guest
//! state.
//! This library is expected to be embedded in a loader environment, e.g. a
//! UEFI runtim service or a Linux kernel module.
//!
//! This library exports the [VCpu structure](struct.VCpu.html), and four functions.
//! The loader environment is expected to allocate and initialize a VCpu for
//! each logical core, and then load the hypervisor.
//! The loader environment may unload the hypervisor or the hypervisor may
//! unload itself.
//!
//! To load the hypervisor:
//! 1. Once globally, call [rustyvisor_load](fn.rustyvisor_load.html)
//! 2. On each logical core, call [rustyvisor_core_load](fn.rustyvisor_core_load.html)
//!
//! To unload the hypervisor after it has been loaded:
//! 1. On each logical core, call [rustyvisor_core_unload](fn.rustyvisor_core_unload.html)
//! 2. Once globally, call [rustyvisor_unload](fn.rustyvisor_unload.html)

use ::log::{error, info, trace, LevelFilter};

mod debug;
pub mod interrupt_controller;
mod interrupts;
mod isr;
mod msr;
mod panic;
mod register_state;
pub mod segmentation;
mod vcpu;
mod vmcs;
mod vmcs_dump;
mod vmcs_fields;
mod vmexit_handlers;
mod vmexit_reasons;
mod vmx;

#[cfg(target_os = "uefi")]
use pcuart::logger;

#[cfg(target_os = "uefi")]
/// Logger used by the hypervisor when loading the guest and while running.
pub static LOGGER: logger::UartLogger = logger::UartLogger::new(pcuart::UartComPort::Com1);
#[cfg(target_os = "uefi")]
/// Logger used by the hypervisor when panicking.
pub static UNSYNCHRONIZED_LOGGER: logger::UnsynchronizedUartLogger =
    logger::UnsynchronizedUartLogger::new(pcuart::UartComPort::Com1);

#[cfg(not(target_os = "uefi"))]
use dmesg_logger as logger;
#[cfg(not(target_os = "uefi"))]
/// Logger used by the hypervisor when loading the guest and while running.
pub static LOGGER: logger::DMesgLogger = logger::DMesgLogger {};
#[cfg(not(target_os = "uefi"))]
/// Logger used by the hypervisor when panicking.
pub static UNSYNCHRONIZED_LOGGER: logger::DMesgLogger = logger::DMesgLogger {};

/// This structure represents all of the data the hypervisor needs for a single CPU.
/// The environment loader, e.g. UEFI bindings or linux kernel bindings, needs
/// to allocate this structure on memory which will be accessible for the
/// lifetime of the hypervisor.
///
/// At present we represent fields as C-style pointers for virtual addresses
/// usable by the host instead of rust style references, so that we can write
/// the environment loader in another language like C.
/// Physical addresses are represented as u64s.
#[derive(Debug)]
#[repr(C)]
pub struct VCpu {
    /// A pointer to the virtual address representing this VCpu structure.
    /// In hypervisor host context we will set the fs base to point to this
    /// structure. This way we can get a pointer to this structure by accessing
    /// fs:0. See [get_current_vcpu](vcpu/fn.get_current_vcpu.html) for more
    /// details.
    pub this_vcpu: *mut VCpu,
    /// The virtual address of this core's vmx on region. This is used by the
    /// hardware as scratch space and its contents are largely opaque to the
    /// hypervisor. Must be at least a page.
    pub vmxon_region: *mut u32,
    /// A pointer to this core's virtual machine control structure.
    /// The virtual machine control structure, or vmcs, is used to control the
    /// state of the processor when entering and exiting virtualization, as
    /// well as control what events cause vm exits and the hardware
    /// capabilitiesexposed by the hypervisor. Must be at least a page.
    pub vmcs: *mut u32,
    /// The physical address of the vmxon region. Must back the vmxon_region
    /// virtual address above.
    pub vmxon_region_phys: u64,
    /// The physical address of the vmcs. Must back the vmxon_region
    /// virtual address above.
    pub vmcs_phys: u64,
    /// Size in bytes of the vmx on region. Must be at least a page.
    pub vmxon_region_size: usize,
    /// Size in bytes of the vmcs. Must be at least a page.
    pub vmcs_size: usize,
    /// True if the hypervisor loaded successfully on this core, false
    /// otherwise.
    pub loaded_successfully: bool,
    /// The virtual address of the base of the hypervisor host's stack.
    pub stack_base: *mut u8,
    /// The size of the hypervisor host's stack.
    pub stack_size: usize,
    /// The virtual address of the top of the hypervisor host's stack.
    pub stack_top: *mut u8,
    /// The base address of the host's global descriptor table. Get using the
    /// instruction sgdt.
    pub host_gdt_base: *mut u64,
    /// The limit, or size in bytes minus one, of the host's global descriptor
    /// table. Get using the instruction sgdt.
    pub host_gdt_limit: u64,
    /// The virtual address of the host's virtual interrupt controller. Must be
    /// initialized as zeroes.
    pub virtual_local_interrupt_controller:
        *mut interrupt_controller::VirtualLocalInterruptController,
    /// The physical address of the 4k/1 page MSR bitmap on processes which
    /// support MSR bitmaps.
    /// The backing memory must be zeroed.
    pub msr_bitmap: u64,
    /// The virtual address of the base of the TSS, a mostly vestigal structure
    /// required by the CPU for hardware task switching.
    pub tr_base: u64,
    /// The selector of the TSS segment.
    pub tr_selector: u16,
}

/// Set up hypervisor global state. Must be one called only once by the loader
/// before rustyvisor_core_load is called. Sets up the logger, the Interrupt
/// Descriptor Table and any host state used for every core.
/// This function must be called at most once per load of the hypervisor and in
/// the following sequence of calls:
/// 1. Once globally, call [rustyvisor_load](fn.rustyvisor_load.html)
/// 2. On each logical core, call [rustyvisor_core_load](fn.rustyvisor_core_load.html)
/// 3. On each logical core, call [rustyvisor_core_unload](fn.rustyvisor_core_unload.html)
/// 4. Once globally, call [rustyvisor_unload](fn.rustyvisor_unload.html)
/// This function may be called on any processor, not just the bootstrap
/// processor.
#[no_mangle]
pub extern "C" fn rustyvisor_load() -> i32 {
    let logger_result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));
    match logger_result {
        Ok(()) => {}
        Err(_) => return -1,
    }

    info!("{}", "rustyvisor_load");

    interrupts::init_interrupt_handlers(x86::segmentation::cs().bits());

    #[cfg(feature = "runtime_tests")]
    runtime_tests();

    0
}

/// Load the hypervisor on the current logical core.
/// Enables VMX on the current core and enters vmx guest operation.
/// After returning, the caller will be running as a VM guest.
/// This function must be called at most once per-core and in the following
/// sequence of calls:
/// 1. Once globally, call [rustyvisor_load](fn.rustyvisor_load.html)
/// 2. On each logical core, call [rustyvisor_core_load](fn.rustyvisor_core_load.html)
/// 3. On each logical core, call [rustyvisor_core_unload](fn.rustyvisor_core_unload.html)
/// 4. Once globally, call [rustyvisor_unload](fn.rustyvisor_unload.html)
#[no_mangle]
pub extern "C" fn rustyvisor_core_load(data: &VCpu) -> i32 {
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

/// Unload the hypervisor from the current logical core. Disables VMX operation once
/// the hypervisor is unloaded. When this function returns, the core will be
/// running in VMX root operation, i.e. not as a VM guest.
/// This function must be called at most once per-core and in the following
/// sequence of calls:
/// 1. Once globally, call [rustyvisor_load](fn.rustyvisor_load.html)
/// 2. On each logical core, call [rustyvisor_core_load](fn.rustyvisor_core_load.html)
/// 3. On each logical core, call [rustyvisor_core_unload](fn.rustyvisor_core_unload.html)
/// 4. Once globally, call [rustyvisor_unload](fn.rustyvisor_unload.html)
#[no_mangle]
pub extern "C" fn rustyvisor_core_unload() {
    info!("Core unload");
    vmx::unload_vm();
    vmx::disable();
}

/// Tear down hypervisor global state.
/// This function must be called at most once per load of the hypervisor and in
/// the following sequence of calls:
/// 1. Once globally, call [rustyvisor_load](fn.rustyvisor_load.html)
/// 2. On each logical core, call [rustyvisor_core_load](fn.rustyvisor_core_load.html)
/// 3. On each logical core, call [rustyvisor_core_unload](fn.rustyvisor_core_unload.html)
/// 4. Once globally, call [rustyvisor_unload](fn.rustyvisor_unload.html)
/// This function may be called on any processor, not just the bootstrap
/// processor.
#[no_mangle]
pub extern "C" fn rustyvisor_unload() {
    info!("Hypervisor unloaded.");
}

/// Stub for hypervisor runtime tests.
#[cfg(feature = "runtime_tests")]
fn runtime_tests() {
    info!("Executing runtime tests...");
    info!("Runtime tests succeeded");
}
