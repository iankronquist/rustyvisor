#![no_std]
#![feature(asm)]
#![warn(missing_docs)]

use core::fmt;
pub mod logger;

#[derive(Copy, Clone)]
#[repr(u16)]
pub enum UartComPort {
    Com1 = 0x3f8,
    Com2 = 0x2f8,
    Com3 = 0x3e8,
    Com4 = 0x2e8,
}

const UART_OFFSET_TRANSMITTER_HOLDING_BUFFER: u16 = 0;
//const UART_OFFSET_RECEIVER_BUFFER: u16 = 0;
const UART_OFFSET_DIVISOR_LATCH_LOW: u16 = 0;
const UART_OFFSET_INTERRUPT_ENABLE: u16 = 1;
const UART_OFFSET_DIVISOR_LATCH_HIGH: u16 = 1;
//const UART_OFFSET_INTERRUPT_IDENTIFICATION: u16 = 2;
const UART_OFFSET_FIFO_CONTROL: u16 = 2;
const UART_OFFSET_LINE_CONTROL: u16 = 3;
const UART_OFFSET_MODEM_CONTROL: u16 = 4;
const UART_OFFSET_LINE_STATUS: u16 = 5;
//const UART_OFFSET_MODEM_STATUS: u16 = 6;
//const UART_OFFSET_SCRATCH: u16 = 7;

#[derive(Default)]
pub struct Uart {
    io_port_base: u16,
}

#[derive(Copy, Clone)]
pub enum UartBaudRate {
    Baud115200 = 115200,
    Baud9600 = 9600,
}

impl Uart {
    pub const fn new(com: UartComPort) -> Self {
        Self {
            io_port_base: com as u16,
        }
    }
    pub fn init(&self, enable_receiver_interrupts: bool, baud_rate: UartBaudRate) {
        outw(self.io_port_base + UART_OFFSET_INTERRUPT_ENABLE, 0x00);
        outw(self.io_port_base + UART_OFFSET_LINE_CONTROL, 0x80);

        let dlab_low: u16 = baud_rate as u16 & 0xff;
        let dlab_high: u16 = (baud_rate as u16 >> 8) & 0xff;
        outw(self.io_port_base + UART_OFFSET_DIVISOR_LATCH_LOW, dlab_low);
        outw(
            self.io_port_base + UART_OFFSET_DIVISOR_LATCH_HIGH,
            dlab_high,
        );
        outw(self.io_port_base + UART_OFFSET_LINE_CONTROL, 0x03);
        outw(self.io_port_base + UART_OFFSET_FIFO_CONTROL, 0xc7);
        outw(self.io_port_base + UART_OFFSET_MODEM_CONTROL, 0x0b);
        if enable_receiver_interrupts {
            //outw(self.io_port_base + UART_OFFSET_INTERRUPT_ENABLE, 0x01);
            unimplemented!();
        } else {
            outw(self.io_port_base + UART_OFFSET_INTERRUPT_ENABLE, 0x00);
        }
    }
}

fn outw(port: u16, data: u16) {
    unsafe { x86::io::outw(port, data) }
}

fn outb(port: u16, data: u8) {
    unsafe { x86::io::outb(port, data) }
}

fn inb(port: u16) -> u8 {
    unsafe { x86::io::inb(port) }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            while (inb(self.io_port_base + UART_OFFSET_LINE_STATUS) & 0x20) == 0 {}
            outb(
                self.io_port_base + UART_OFFSET_TRANSMITTER_HOLDING_BUFFER,
                c as u8,
            );
        }
        Ok(())
    }
}
