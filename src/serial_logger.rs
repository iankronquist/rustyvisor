#![allow(dead_code)]
use core::fmt;
use core::fmt::Write;
use spin::Mutex;

#[derive(PartialOrd, PartialEq, Eq)]
#[derive(Debug)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

const PORT: u16 = 0x3f8;

#[derive(Default)]
pub struct SerialPort;

pub struct SerialLogger {
    port: SerialPort,
    level: Level
}


pub static SERIAL_PORT: Mutex<SerialLogger> = Mutex::new(SerialLogger { port: SerialPort{}, level: Level::Debug });

fn outw(port: u16, data: u16) {
    unsafe {
        asm!("outw %ax, %dx" : : "{dx}" (port as u16), "{ax}" (data));
    }
}

fn outb(port: u16, data: u8) {
    unsafe {
        asm!("outb %al, %dx" : : "{dx}" (port as u16), "{al}" (data));
    }
}

fn inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("inb %dx, %al" : "={al}"(data) : "{dx}"(port as u16));
    }
    data
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            while (inb(PORT + 5) & 0x20) == 0 {}
            outb(PORT, c as u8);
        }
        Ok(())
    }
}

impl fmt::Write for SerialLogger {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        SERIAL_PORT.lock().port.write_str(s)
    }
}

impl SerialLogger {
    pub fn init(&self) -> Result<(), ()> {
        outw(PORT + 1, 0x00);
        outw(PORT + 3, 0x80);
        outw(PORT, 0x01);
        outw(PORT + 1, 0x00);
        outw(PORT + 3, 0x03);
        outw(PORT + 2, 0xc7);
        outw(PORT + 4, 0x0b);
        outw(PORT + 1, 0x01);
        Ok(())
    }

    pub fn is_enabled(&self, level: Level) -> bool {
        level >= self.level
    }

    pub fn log(&mut self, level: Level, fmt: fmt::Arguments) {
        let _ = write!(self, "{:?}: {}\n", level, fmt);
    }
}

#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt:tt, $($arg:tt)*) => {
        {
            use logger;
            let mut ul = logger::SERIAL_PORT.lock();
            if ul.is_enabled($lvl) {
                ul.log($lvl, format_args!($fmt, $($arg)*));
            }
        }
    };
    ($lvl:expr, $fmt:tt) => {
        {
            use logger;
            let mut ul = logger::SERIAL_PORT.lock();
            if ul.is_enabled($lvl) {
                ul.log($lvl, format_args!($fmt));
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:tt, $($arg:tt)*) => {
        log!(logger::Level::Error, $fmt, $($arg)*);
    };
    ($fmt:tt) => {
        log!(logger::Level::Error, $fmt);
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:tt, $($arg:tt)*) => {
        log!(logger::Level::Info, $fmt, $($arg)*);
    };
    ($fmt:tt) => (
        log!(logger::Level::Info, $fmt);
    );
}

#[macro_export]
macro_rules! warn {
    ($fmt:tt, $($arg:tt)*) => (
        log!(logger::Level::Warn, $fmt, $($arg)*);
    );
    ($fmt:tt) => (
        log!(logger::Level::Warn, $fmt);
    )
}

#[macro_export]
macro_rules! debug {
    ($fmt:tt, $($arg:tt)*) => (
        log!(logger::Level::Debug, $fmt, $($arg)*);
    );
    ($fmt:tt) => (
        log!(logger::Level::Debug, $fmt);
    )
}

pub fn init() -> Result<(), ()> {
    let sp = SERIAL_PORT.lock();
    (*sp).init()
}

pub fn fini() { }


// Used in panic handler.
pub unsafe fn bust_locks() {
    SERIAL_PORT.force_unlock();
}

/*
impl log::Log for SerialLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            let _ = write!(
                SERIAL_PORT.lock(),
                "{}: {}\r\n",
                record.level(),
                record.args()
            );
        }
    }
}

pub fn init() -> Result<(), log::SetLoggerError> {
    unsafe {
        log::set_logger_raw(|max_log_level| {
            static LOGGER: SerialLogger = SerialLogger;
            LOGGER.init();
            max_log_level.set(log::LevelFilter::Debug);
            &LOGGER
        })
    }
}

pub fn fini() -> Result<(), log::ShutdownLoggerError> {
    log::shutdown_logger_raw().map(|_logger| {})
}
*/
