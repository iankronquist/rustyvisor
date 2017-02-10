#![no_std]
#![feature(alloc)]
#![feature(asm)]
#![feature(collections)]
#![feature(const_fn)]
#![feature(integer_atomics)]
#![feature(lang_items)]

#![allow(unknown_lints)]

extern crate alloc;
#[cfg(not(test))]
extern crate allocator;
extern crate spin;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate collections;
#[macro_use]
extern crate log;

pub mod cpu;
mod dispatch_table;
pub mod hash_map;
pub mod interrupts;
mod isr;
pub mod os;
pub mod paging;
pub mod runtime;
pub mod segmentation;
pub mod vmx;

#[cfg(not(test))]
mod serial_logger;


include!(concat!(env!("OUT_DIR"), "/version.rs"));


#[no_mangle]
pub extern "C" fn rustyvisor_load(kernel_data: &mut os::KernelData) -> u32 {

    cpu::init(1);
    cpu::bring_core_online();

    if cpu::get_number() == 0 {
        #[cfg(not(test))]
        {
            allocator::init(kernel_data.heap_size, kernel_data.heap);
            match serial_logger::init() {
                Ok(()) => {}
                Err(_e) => return 1,
            }
        }

        info!("{}", VERSION);
    }


    #[cfg(feature = "runtime_tests")]
    runtime_tests(kernel_data);

    0
}

#[no_mangle]
pub extern "C" fn rustyvisor_unload() {
    #[cfg(not(test))]
    {
        let _ = serial_logger::fini();
    }
}

#[cfg(feature = "runtime_tests")]
fn runtime_tests(kernel_data: &mut os::KernelData) {
    info!("Executing runtime tests...");

    cpu::runtime_tests::run();
    segmentation::runtime_tests::run();
    interrupts::runtime_tests::run();
    interrupts::cli::runtime_tests::run();
    paging::runtime_tests::run(kernel_data.translations,
        kernel_data.translations_count);

    info!("Runtime tests succeeded");
}
