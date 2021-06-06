use log;

use super::{Uart, UartBaudRate, UartComPort};
use core::fmt::Write;
use spin::Mutex;

pub struct UartLogger {
    port: Mutex<Uart>,
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

    pub unsafe fn bust_locks(&self) {
        self.port.force_unlock();
    }

    fn lock_port_with_timeout(&self) -> spin::MutexGuard<Uart> {
        let timeout = 0x1000;
        let mut count = 0;
        while count < timeout {
            if let Some(guard) = self.port.try_lock() {
                return guard;
            }
            count += 1;
        }
        unsafe {
            self.bust_locks();
        }
        panic!("Timeout exceeded");
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
                self.lock_port_with_timeout(),
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
