#![feature(allocator)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]

pub mod runtime;
pub mod allocator;

pub mod vmx;


pub type CChar = u8;

macro_rules! cstring {
    ($e:expr) => (concat!($e, "\0").as_ptr() as *const CChar)
}

extern "C" {
    fn printk(format: *const CChar, ...);
}



#[no_mangle]
pub extern "C" fn entry(heap: *mut CChar, heap_size: u64, _: *mut CChar, _: u64) -> u32 {
    unsafe {
        printk(cstring!("Hello Linux!\n"));
    }
    allocator::init_global_allocator(heap_size, heap);
    return 0;
}
