#![feature(lang_items)]
#![feature(asm)]
#![no_std]


mod runtime;
pub mod vmx;


pub type CChar = i8;

macro_rules! cstring {
    ($e:expr) => (concat!($e, "\0").as_ptr() as *const CChar)
}

extern "C" {
    fn printk(format: *const CChar, ...);
}


#[no_mangle]
pub extern "C" fn entry() {
    unsafe {
        printk(cstring!("Hello Linux!\n"));
    }
}
