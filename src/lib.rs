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
extern crate collections;

#[macro_use]
pub mod linux;

pub mod cli;
pub mod dispatch_table;
pub mod gdt;
pub mod hash_map;
pub mod runtime;
pub mod vmx;

#[no_mangle]
pub extern "C" fn entry(_heap: *mut u8, _heap_size: u64, _: *mut u8, _: u64) -> u32 {
    unsafe {
        linux::printk(cstring!("Hello Linux!\n"));
    }

    #[cfg(feature = "runtime_tests")]
    runtime_tests();


    #[cfg(not(test))]
    allocator::init_global_allocator(_heap_size, _heap);
    0
}

#[cfg(feature = "runtime_tests")]
fn runtime_tests() {
    gdt::runtime_tests::test_load();
}
