use dispatch_table::{DispatchTable, DispatchFn};
use spin::RwLock;
use isr;

#[derive(Copy, Clone, Default)]
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

pub struct IDT([IDTEntry; 256]);


lazy_static! {
    static ref INTERRUPT_TABLE: RwLock<DispatchTable<u64, InterruptCPUState>> =
        RwLock::new(DispatchTable::new(256));
    static ref DESCRIPTOR_TABLE: RwLock<IDT> = RwLock::new(IDT([Default::default(); 256]));
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
                                  stub: isr::InterruptServiceRoutine,
                                  handler: DispatchFn<u64, InterruptCPUState>) {
    INTERRUPT_TABLE.write().register(interrupt, handler);
    DESCRIPTOR_TABLE.write().set_entry(interrupt as usize, stub as u64, 0x08, 0x8e);
}

pub fn unregister_interrupt_handler(interrupt: u64) {
    INTERRUPT_TABLE.write().unregister(interrupt);
}

#[no_mangle]
pub extern "C" fn dispatch_interrupt(state: &mut InterruptCPUState) {
    INTERRUPT_TABLE.read().dispatch(state.interrupt_number, state);
}

fn default_handler(_interrupt_number: u64, _regs: &mut InterruptCPUState) -> bool {
    true
}


pub fn init_interrupt_handlers() {
    for i in 0..20 {
        register_interrupt_handler(i, isr::ISR[i as usize], default_handler);
    }
}



#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {

    use interrupts;
    use vmx;
    use cli;
    use isr;
    use core::mem;

    extern "C" {
        fn _test_division_by_zero_routine() -> bool;

        // DO NOT CALL THIS FUNCTION. YOU WILL MESS UP THE STACK.
        fn _after_division() -> bool;
    }

    fn new_host_idt_descriptor() -> vmx::CPUTableDescriptor {
        let base = (*interrupts::DESCRIPTOR_TABLE.write()).0.as_ptr() as u64;
        vmx::CPUTableDescriptor {
            base: base,
            limit: (mem::size_of::<interrupts::IDT>() - 1) as u16,
        }
    }


    pub fn run() {
        test_load_and_restore_idt();
        test_divide_by_zero_interrupt();
    }

    fn test_load_and_restore_idt() {
        cli::ClearLocalInterruptsGuard::new();
        let idt_desc = new_host_idt_descriptor();
        let mut orig_idt_desc: vmx::CPUTableDescriptor = Default::default();
        vmx::sidt(&mut orig_idt_desc);
        vmx::lidt(&idt_desc);
        vmx::lidt(&orig_idt_desc);
    }

    fn division_by_zero_handler(interrupt_number: u64,
                                regs: &mut interrupts::InterruptCPUState)
                                -> bool {
        assert_eq!(interrupt_number, 0);
        info!("Handling division by zero interrupt.");
        // A div instruction is three bytes long. If we return without
        // advancing the rip we'll execute the same instruction again and wind
        // up in a fault loop.
        regs.rip = _after_division as u64;
        true
    }

    #[allow(unused_variables)]
    fn test_divide_by_zero_interrupt() {
        let mut orig_idt_desc: vmx::CPUTableDescriptor = Default::default();
        interrupts::register_interrupt_handler(0, isr::_isr0, division_by_zero_handler);

        {
            cli::ClearLocalInterruptsGuard::new();
            let idt_desc = new_host_idt_descriptor();
            vmx::sidt(&mut orig_idt_desc);
            vmx::lidt(&idt_desc);
        }

        info!("Here we go!");

        unsafe {
            assert!(_test_division_by_zero_routine());
        }

        info!("Successfully returned from a division by zero interrupt.");

        {
            cli::ClearLocalInterruptsGuard::new();
            vmx::lidt(&orig_idt_desc);
        }


    }
}
