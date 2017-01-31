use dispatch_table::{DispatchTable, DispatchFn};
use spin::RwLock;
use core::mem;
use cli;
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


#[derive(Default)]
pub struct IDT([IDTEntry; 20]);


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


pub fn lidt(idt_desc: *const IDTDescriptor) {
    unsafe {
        asm!(
            "lidt ($0)"
            :
            : "r"(idt_desc)
            :
            );
    }
}


pub fn sidt(idt_desc: *mut IDTDescriptor) {
    unsafe {
        asm!(
            "sidt ($0)"
            :
            : "r"(idt_desc)
            :
            );
    }
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


pub struct InterruptTable {
    descriptor_table: cli::ClearLocalInterruptsGuard<IDT>,
    dispatch_table: DispatchTable<u64, InterruptCPUState>,
}


impl Default for InterruptTable {
    fn default() -> Self {
        InterruptTable {
            descriptor_table: Default::default(),
            dispatch_table: DispatchTable::new(20),
        }
    }
}

impl InterruptTable {
    fn as_ptr(&self) -> *const IDT {
        (*self.descriptor_table).0.as_ptr() as *const IDT
    }
}


lazy_static! {
    static ref INTERRUPT_TABLE: RwLock<InterruptTable> = Default::default();
}


pub fn register_interrupt_handler(interrupt: u64,
                                  stub: isr::InterruptServiceRoutine,
                                  handler: DispatchFn<u64, InterruptCPUState>) {
    let mut table = INTERRUPT_TABLE.write();
    table.dispatch_table.register(interrupt, handler);
    table.descriptor_table.set_entry(interrupt as usize, stub as u64, 0x08, 0x8e);
}


pub fn unregister_interrupt_handler(interrupt: u64) {
    let mut table = INTERRUPT_TABLE.write();
    table.dispatch_table.unregister(interrupt);
    table.descriptor_table.set_entry(interrupt as usize, 0, 0, 0);
}


#[no_mangle]
pub extern "C" fn dispatch_interrupt(state: &mut InterruptCPUState) {
    INTERRUPT_TABLE.read().dispatch_table.dispatch(state.interrupt_number, state);
}


fn default_handler(_interrupt_number: u64, _regs: &mut InterruptCPUState) -> bool {
    true
}


pub fn init_interrupt_handlers() {
    for i in 0..20 {
        register_interrupt_handler(i, isr::ISR[i as usize], default_handler);
    }
}


#[derive(Default)]
#[repr(packed)]
pub struct IDTDescriptor {
    pub limit: u16,
    pub base: u64,
}


impl IDTDescriptor {
    pub fn new() -> cli::ClearLocalInterruptsGuard<IDTDescriptor> {
        cli::ClearLocalInterruptsGuard::new(IDTDescriptor {
            limit: (mem::size_of::<IDT>() - 1) as u16,
            base: (*INTERRUPT_TABLE).read().as_ptr() as u64,
        })
    }

    pub fn from_cpu() -> cli::ClearLocalInterruptsGuard<IDTDescriptor> {
        let mut current_idt_ptr: IDTDescriptor = Default::default();
        sidt(&mut current_idt_ptr);
        cli::ClearLocalInterruptsGuard::new(current_idt_ptr)
    }

    pub fn load(&self) {
        lidt(self);
    }
}


#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {

    use interrupts::IDTDescriptor;

    pub fn run() {
        test_load_and_restore_idt();
    }

    fn test_load_and_restore_idt() {
        let orig_idt_desc = IDTDescriptor::from_cpu();
        let idt_desc = IDTDescriptor::new();
        idt_desc.load();
        orig_idt_desc.load();
    }
}
