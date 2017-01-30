#![feature(collections)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

#![feature(alloc)]
#![allow(unknown_lints)]
#![no_std]

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

#[macro_use]
pub mod linux;

pub mod cli;
pub mod dispatch_table;
pub mod gdt;
pub mod hash_map;
pub mod interrupts;
pub mod isr;
pub mod runtime;
pub mod vmx;
pub mod serial_logger;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[no_mangle]
pub extern "C" fn rustyvisor_load(_heap: *mut u8, _heap_size: u64, _: *mut u8, _: u64) -> u32 {
    #[cfg(not(test))]
    {
        allocator::init_global_allocator(_heap_size, _heap);
        match serial_logger::init() {
            Ok(()) => {}
            Err(_e) => return 1,
        }
    }

    info!("{}", VERSION);

    #[cfg(feature = "runtime_tests")]
    runtime_tests();

    0
}

#[no_mangle]
pub extern "C" fn rustyvisor_unload() {
     let _ = serial_logger::shutdown();
}

#[cfg(feature = "runtime_tests")]
fn runtime_tests() {
    info!("Executing runtime tests...");

    gdt::runtime_tests::run();
    interrupts::runtime_tests::run();

    info!("Runtime tests succeeded");
}
