use linux;
use log;


use log::{LogRecord, LogLevel, LogMetadata};

#[derive(Default)]
struct DMesgLogger(());

impl log::Log for DMesgLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let log_message = format!("{}: {}", record.level(), record.args());
            unsafe {
                linux::printk(cstring!("%s\n"), log_message);
            }
        }
    }

}

pub fn init() -> Result<(), log::SetLoggerError> {
    unsafe {
        log::set_logger_raw(|max_log_level| {
            static LOGGER: DMesgLogger = DMesgLogger(());
            max_log_level.set(log::LogLevelFilter::Info);
            &LOGGER
        })
    }
}
pub fn shutdown() -> Result<(), log::ShutdownLoggerError> {
    log::shutdown_logger_raw().map(|_logger| { })
}
