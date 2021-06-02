use log;

use core::fmt::{self, Write};
use spin::Mutex;
use super::{UartComPort, Uart, UartBaudRate};

pub struct UartLogger {
    port: Mutex<Uart>,
}

impl fmt::Write for UartLogger {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let mut port = self.port.lock();
        port.write_str(s)
    }
}

impl UartLogger {
    pub const fn new(port: UartComPort) -> Self {
        Self { port: Mutex::new(Uart::new(port)) }
    }
    pub fn init(&self) -> Result<(), log::SetLoggerError> {
        self.port.lock().init(false, UartBaudRate::Baud115200);
        Ok(())
    }
}

impl log::Log for UartLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Info
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            let _ = writeln!(
                self.port.lock(),
                "{}: {}",
                record.level(),
                record.args()
            );
        }
    }
}

pub fn fini() -> Result<(), log::ShutdownLoggerError> {
    log::shutdown_logger_raw().map(|_logger| {})
}

pub static LOGGER: UartLogger = UartLogger { port: Mutex::new(Uart::new(UartComPort::Com1))};