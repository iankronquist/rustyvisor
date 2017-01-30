use log;
use collections::String;

const PORT: u16 = 0x3f8;

struct SerialLogger(());

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

impl SerialLogger {
    fn init(&self) {
        outw(PORT + 1, 0x00);
        outw(PORT + 3, 0x80);
        outw(PORT, 0x01);
        outw(PORT + 1, 0x00);
        outw(PORT + 3, 0x03);
        outw(PORT + 2, 0xc7);
        outw(PORT + 4, 0x0b);
        outw(PORT + 1, 0x01);
    }

    fn write_string(&self, s: String) {
        for c in s.chars() {
            while (inb(PORT + 5) & 0x20) == 0 {}
            outb(PORT, c as u8);
        }
    }
}

impl log::Log for SerialLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Info
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            let log_message = format!("{}: {}\n", record.level(), record.args());
            self.write_string(log_message);
        }
    }
}

pub fn init() -> Result<(), log::SetLoggerError> {
    unsafe {
        log::set_logger_raw(|max_log_level| {
            static LOGGER: SerialLogger = SerialLogger(());
            LOGGER.init();
            max_log_level.set(log::LogLevelFilter::Info);
            &LOGGER
        })
    }
}

pub fn shutdown() -> Result<(), log::ShutdownLoggerError> {
    log::shutdown_logger_raw().map(|_logger| {})
}
