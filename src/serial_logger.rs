use log;
use log::{LogRecord, LogLevel, LogMetadata};
use collections::String;

#[repr(u16)]
enum SerialPortRegister {
    COM1Data = 0x3f8,
    COM1InterruptControl = 0x3f9,
    //COM1InterruptIDAndFIFO = 0x3fa,
    COM1LineControl = 0x3fb,
    COM1ModemControl = 0x3fc,
    COM1LineStatus = 0x3fd,
    //COM1ModemStatus = 0x3fe,
    //COM1Scratch = 0x3ff,
}

fn outw(port: SerialPortRegister, data: u16) {
    unsafe {
        asm!("outw %ax, %dx" : : "{dx}" (port as u16), "{ax}" (data));
    }
}

fn outb(port: SerialPortRegister, data: u8) {
    unsafe {
        asm!("outb %al, %dx" : : "{dx}" (port as u16), "{al}" (data));
    }
}

fn inb(port: SerialPortRegister) -> u8 {
    let data: u8;
    unsafe {
        asm!("inb %dx, %al" : "={al}"(data) : "{dx}"(port as u16));
    }
    data
}


impl SerialLogger {
    fn init(&self) {
        outw(SerialPortRegister::COM1InterruptControl, 0x00);
        outw(SerialPortRegister::COM1ModemControl, 0x80);
        outw(SerialPortRegister::COM1Data, 0x01);
        outw(SerialPortRegister::COM1InterruptControl, 0x00);
        outw(SerialPortRegister::COM1ModemControl, 0x03);
        outw(SerialPortRegister::COM1LineControl, 0xc7);
        outw(SerialPortRegister::COM1LineStatus, 0x0b);
        outw(SerialPortRegister::COM1InterruptControl, 0x01);

        /*
        outw(SerialPortRegister::COM1LineControl, 0x83);
        outw(SerialPortRegister::COM1Data, 0x01);
        outw(SerialPortRegister::COM1InterruptControl, 0x00);
        outw(SerialPortRegister::COM1InterruptControl, 0x03);
        outw(SerialPortRegister::COM1ModemControl, 0x03);
        inb(SerialPortRegister::COM1Data);
        */
    }

    fn write_string(&self, s: String) {
        for c in s.chars() {
            while (inb(SerialPortRegister::COM1LineStatus) & 0x20) != 0 {}
            outb(SerialPortRegister::COM1Data, c as u8);
        }
    }
}


#[derive(Default)]
struct SerialLogger(());

impl log::Log for SerialLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let log_message = format!("{}: {}\0", record.level(), record.args());
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
