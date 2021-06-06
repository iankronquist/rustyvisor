use log;

use super::{Uart, UartBaudRate, UartComPort};
use core::fmt::Write;
use spin::Mutex;

pub struct UartLogger {
    port: Mutex<Uart>,
}

/// Allows direct write access to a uart COM port without any synchronization.
/// Does not allow the COM port to be reconfigured.
/// This is safe because at the end of the day it's just a series of outb instructions, there are no possible memory safety issues.
/// Messages can get jumbled together, but that's not necessarily an issue.
/// This is useful during panics, when we want to try to write out to the serial port without causing a potential deadlock.
pub struct UnsynchronizedUartLogger {
    port: UartComPort,
}

impl UnsynchronizedUartLogger {
    /// Creates a new UnsynchronizedUartLogger for the provided serial port.
    pub const fn new(port: UartComPort) -> Self {
        Self {
            port,
        }
    }
}

impl UnsynchronizedUartLogger {
    /// Write the formatter directly to the UART COM port without any synchronization.
    /// Has a signature very similar to one of the methods in core::fmt::Write so that
    /// we can call it via the write! macro, but self is not mutable so we can safely
    /// call it on global static variables without something like a mutex.
    pub fn write_fmt(&self, args: core::fmt::Arguments) {
        let mut uart = Uart::new(self.port);
        let _ = write!(uart, "{}\r\n", args);
    }
}

impl UartLogger {
    pub const fn new(port: UartComPort) -> Self {
        Self {
            port: Mutex::new(Uart::new(port)),
        }
    }
    pub fn init(&self) {
        self.port.lock().init(false, UartBaudRate::Baud115200);
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
        panic!("Timeout exceeded");
    }
}

impl log::Log for UartLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
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

impl log::Log for UnsynchronizedUartLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut uart = Uart::new(self.port);
            let _ = write!(
                uart,
                "{}: {}\r\n",
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

