
type InterruptHandlerFn = unsafe extern "C" fn() -> !;

use dispatch_table::{DispatchTable, DispatchFn};
use spin::RwLock;
use vmx;
use core::mem;

#[derive(Default)]
#[repr(packed)]
struct IDTEntry {
    base_low: u16,
    selector: u16,
    always0: u8,
    flags: u8,
    base_high: u16,
    base_highest: u32,
    _reserved: u32,
}

fn new_host_idt_descriptor() -> vmx::CPUTableDescriptor {
    let base = (*DESCRIPTOR_TABLE.write()).0.as_ptr() as u64;
    vmx::CPUTableDescriptor {
        base: base,
        limit: (mem::size_of::<IDT>() - 1) as u16,
    }
}

#[derive(Default)]
pub struct IDT([IDTEntry; 20]);

lazy_static! {
    static ref INTERRUPT_TABLE: RwLock<DispatchTable<u64>> = RwLock::new(DispatchTable::new(256));
    static ref DESCRIPTOR_TABLE: RwLock<IDT> = RwLock::new(Default::default());
}


impl IDT {
    fn set_entry(&mut self, num: usize, base: u64, selector: u16, flags: u8) {
        self.0[num].base_low = (base & 0xffff) as u16;
        self.0[num].base_high = ((base >> 16) & 0xffff) as u16;
        self.0[num].base_highest = (base >> 32) as u32;
        self.0[num].always0 = 0;
        self.0[num].selector = selector;
        self.0[num].flags = flags;
    }
}

#[repr(C)]
pub struct InterruptCPUState {
    ds: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
    interrupt_number: u64,
    error_code: u64,
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

pub fn register_interrupt_handler(interrupt: u64,
                                  stub: InterruptHandlerFn,
                                  handler: DispatchFn<u64>) {
    INTERRUPT_TABLE.write().register(interrupt, handler);
    DESCRIPTOR_TABLE.write().set_entry(interrupt as usize, stub as u64, 0x08, 0x8e);
}

pub fn unregister_interrupt_handler(interrupt: u64) {
    INTERRUPT_TABLE.write().unregister(interrupt);
}

#[no_mangle]
pub extern "C" fn dispatch_interrupt(state: &mut InterruptCPUState) {
    INTERRUPT_TABLE.read().dispatch(&state.interrupt_number);
}

fn default_handler(_interrupt_number: &u64) -> bool {
    true
}


extern "C" {
    fn _isr0() -> !;
    fn _isr1() -> !;
    fn _isr2() -> !;
    fn _isr3() -> !;
    fn _isr4() -> !;
    fn _isr5() -> !;
    fn _isr6() -> !;
    fn _isr7() -> !;
    fn _isr8() -> !;
    fn _isr9() -> !;
    fn _isr10() -> !;
    fn _isr11() -> !;
    fn _isr12() -> !;
    fn _isr13() -> !;
    fn _isr14() -> !;
    fn _isr15() -> !;
    fn _isr16() -> !;
    fn _isr17() -> !;
    fn _isr18() -> !;
    fn _isr19() -> !;
}

const ISR: [InterruptHandlerFn; 20] = [_isr0, _isr1, _isr2, _isr3, _isr4, _isr5, _isr6, _isr7,
                                       _isr8, _isr9, _isr10, _isr11, _isr12, _isr13, _isr14,
                                       _isr15, _isr16, _isr17, _isr18, _isr19];

pub fn init_interrupt_handlers() {
    for i in 0..20 {
        register_interrupt_handler(i, ISR[i as usize], default_handler);
    }
}



#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {

    use interrupts;
    use vmx;
    use cli;

    pub fn run() {
        test_load_and_restore_idt();
    }

    fn test_load_and_restore_idt() {
        cli::ClearLocalInterruptsGuard::new();
        let idt_desc = interrupts::new_host_idt_descriptor();
        let mut orig_idt_desc: vmx::CPUTableDescriptor = Default::default();
        vmx::sidt(&mut orig_idt_desc);
        vmx::lidt(&idt_desc);
        vmx::lidt(&orig_idt_desc);
    }

    /*
    fn division_by_zero_handler() {

    }

    fn test_divide_by_zero_interrupt() {
        let mut orig_idt_desc: vmx::CPUTableDescriptor = Default::default();

        {
            cli::ClearLocalInterruptsGuard::new();
            let idt_desc = interrupts::new_host_idt_descriptor();
            vmx::lidt(&idt_desc);
        }

        let a = 10;
        let b = 0;
        let c = a / b; // Division by zero interrupt

    }
    */
}
