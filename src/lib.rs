#![feature(collections)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

#![feature(alloc, collections)]
#![allow(unknown_lints)]
#![no_std]

extern crate alloc;
#[cfg(not(test))]
extern crate allocator;
extern crate spin;
#[macro_use]
extern crate collections;

pub mod dispatch_table;
pub mod runtime;

pub mod vmx;


pub type CChar = u8;

macro_rules! cstring {
    ($e:expr) => (concat!($e, "\0").as_ptr() as *const CChar)
}

extern "C" {
    fn printk(format: *const CChar, ...);
}



#[no_mangle]
pub extern "C" fn entry(_heap: *mut CChar, _heap_size: u64, _: *mut CChar, _: u64) -> u32 {
    unsafe {
        printk(cstring!("Hello Linux!\n"));
    }

    #[cfg(not(test))]
    allocator::init_global_allocator(_heap_size, _heap);
    0
}
