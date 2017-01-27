pub type CChar = u8;

#[macro_export]
macro_rules! cstring {
    ($e:expr) => (concat!($e, "\0").as_ptr() as *const linux::CChar)
}

extern "C" {
    pub fn printk(format: *const CChar, ...);
}
