#![no_std]
use log;

use core::fmt;
use core::fmt::Write;

extern "C" {
    fn printk(fmt: *const u8, ...) -> i32;
}

pub struct DMesgLogger {}

struct PrintK {}

impl fmt::Write for PrintK {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        unsafe {
            printk(b"%s\0".as_ptr(), s.as_ptr());
        }
        Ok(())
    }
}

impl DMesgLogger {
    pub fn write_fmt(&self, args: core::fmt::Arguments) {
        let mut printk_obj = PrintK {};
        let _ = write!(printk_obj, "{}\r\n", args);
    }
}

impl log::Log for DMesgLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.write_fmt(*record.args());
        }
    }

    fn flush(&self) {}
}
