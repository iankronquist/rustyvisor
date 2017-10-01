use linux;
use log;
use core::fmt::Write;
use core::fmt;


use log::{LogRecord, LogLevel, LogMetadata};

#[derive(Default)]
struct DMesgLogger;

impl fmt::Write for DMesgLogger {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        unsafe {
            linux::printk(cstring!("%s"), s.as_ptr());
        }
        Ok(())
    }
}


impl log::Log for DMesgLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let _ = write!(&mut DMesgLogger, "{}", record.args());
        }
    }
}

static LOGGER: DMesgLogger = DMesgLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    unsafe {
        log::set_logger_raw(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Debug);
            &LOGGER
        })
    }
}

pub fn fini() {}

pub fn shutdown() -> Result<(), log::ShutdownLoggerError> {
    log::shutdown_logger_raw().map(|_logger| {})
}

// NULL terminate log messages so they can safely be fed to printk.
#[macro_export]
macro_rules! error {
    (target: $target:expr, $fmt:tt, $($arg:tt)*) => (
        log!(target: $target, log::LogLevel::Error, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt, $($arg:tt)*) => (
        log!(log::LogLevel::Error, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt) => (
        log!(log::LogLevel::Error, concat!($fmt, "\0"));
    )
}



#[macro_export]
macro_rules! warn {
    (target: $target:expr, $fmt:tt, $($arg:tt)*) => (
        log!(target: $target, log::LogLevel::Warn, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt, $($arg:tt)*) => (
        log!(log::LogLevel::Warn, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt) => (
        log!(log::LogLevel::Warn, concat!($fmt, "\0"));
    )
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $fmt:tt, $($arg:tt)*) => (
        log!(target: $target, log::LogLevel::Info, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt, $($arg:tt)*) => (
        log!(log::LogLevel::Info, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt) => (
        log!(log::LogLevel::Info, concat!($fmt, "\0"));
    )
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $fmt:tt, $($arg:tt)*) => (
        log!(target: $target, log::LogLevel::Debug, concat!($fmt, "\0"), $($arg)*);
    ); ($fmt:tt, $($arg:tt)*) => (
        log!(log::LogLevel::Debug, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt) => (
        log!(log::LogLevel::Debug, concat!($fmt, "\0"));
    )
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $fmt:tt, $($arg:tt)*) => (
        log!(target: $target, log::LogLevel::Trace, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt, $($arg:tt)*) => (
        log!(log::LogLevel::Trace, concat!($fmt, "\0"), $($arg)*);
    );
    ($fmt:tt) => (
        log!(log::LogLevel::Trace, concat!($fmt, "\0"));
    )
}
