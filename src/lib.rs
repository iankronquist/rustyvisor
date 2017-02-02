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

mod cli;
pub mod cpu;
pub mod dispatch_table;
pub mod gdt;
pub mod hash_map;
pub mod interrupts;
mod isr;
pub mod runtime;
pub mod vmx;
#[cfg(not(test))]
mod serial_logger;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[no_mangle]
pub extern "C" fn rustyvisor_load(_heap: *mut u8, _heap_size: u64, _: *mut u8, _: u64) -> u32 {

    cpu::init(1);
    cpu::bring_core_online();

    if cpu::get_number() == 0 {
        #[cfg(not(test))]
        {
            allocator::init(_heap_size, _heap);
            match serial_logger::init() {
                Ok(()) => {}
                Err(_e) => return 1,
            }
        }

        info!("{}", VERSION);
    }


    #[cfg(feature = "runtime_tests")]
    runtime_tests();

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
fn runtime_tests() {
    info!("Executing runtime tests...");

    gdt::runtime_tests::run();
    interrupts::runtime_tests::run();

    info!("Runtime tests succeeded");
}
