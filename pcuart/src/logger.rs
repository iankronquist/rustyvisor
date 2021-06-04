use log;

use super::{Uart, UartBaudRate, UartComPort};
use core::fmt::{self, Write};
use spin::Mutex;

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
        Self {
            port: Mutex::new(Uart::new(port)),
        }
    }
    pub fn init(&self) -> Result<(), log::SetLoggerError> {
        self.port.lock().init(false, UartBaudRate::Baud115200);
        Ok(())
    }
}

impl log::Log for UartLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        //metadata.level() <= log::Level::Trace
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let _ = write!(
                self.port.lock(),
                "{}: {}\r\n",
                record.level(),
                record.args()
            );
        }
    }
    fn flush(&self) {}
}

pub fn fini() -> Result<(), log::SetLoggerError> {
    //log::shutdown_logger_raw().map(|_logger| {})
    Ok(())
}

pub static LOGGER: UartLogger = UartLogger {
    port: Mutex::new(Uart::new(UartComPort::Com1)),
};
