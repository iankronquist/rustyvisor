use core::fmt;

const PORT: u16 = 0x3f8;

#[derive(Default)]
pub struct SerialLogger(());

fn inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("inb %dx, %al" : "={al}"(data) : "{dx}"(port as u16));
    }
    data
}

fn outb(port: u16, data: u8) {
    unsafe {
        asm!("outb %al, %dx" : : "{dx}" (port as u16), "{al}" (data));
    }
}

// Serial logger must be configured before calling this method. See full serial
// logger for details.
impl fmt::Write for SerialLogger {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            while (inb(PORT + 5) & 0x20) == 0 {}
            outb(PORT, c as u8);
        }
        Ok(())
    }

    /*
    fn write_fmt(&mut self, args: fmt::Arguments) -> Result<(), fmt::Error> {
        self.write_str(format!(args));
        Ok(())
    }
    */

}
