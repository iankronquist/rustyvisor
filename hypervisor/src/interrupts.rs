use crate::isr;
use spin::Mutex;

use crate::register_state::InterruptCpuState;

#[allow(unused)]
#[derive(Copy, Clone, Default)]
#[repr(packed)]
struct IdtEntry {
    base_low: u16,
    selector: u16,
    always0: u8,
    flags: u8,
    base_high: u16,
    base_highest: u32,
    _reserved: u32,
}

#[derive(Default)]
pub struct Idt([IdtEntry; 20]);

impl IdtEntry {
    const fn new() -> Self {
        IdtEntry {
            base_high: 0,
            base_highest: 0,
            base_low: 0,
            always0: 0,
            flags: 0,
            selector: 0,
            _reserved: 0,
        }
    }
}

const IDT_ENTRY_FLAGS_RING_0: u8 = 0x00;
const IDT_ENTRY_FLAGS_PRESENT: u8 = 0x80;
const IDT_ENTRY_FLAGS_INTERRUPT_GATE: u8 = 0x0e;

impl Idt {
    const fn new() -> Self {
        Idt([IdtEntry::new(); 20])
    }
    fn set_entry(&mut self, num: usize, base: u64, selector: u16, flags: u8) {
        self.0[num].base_low = (base & 0xffff) as u16;
        self.0[num].base_high = ((base >> 16) & 0xffff) as u16;
        self.0[num].base_highest = (base >> 32) as u32;
        self.0[num].always0 = 0;
        self.0[num].selector = selector;
        self.0[num].flags = flags;
    }
}

static IDT: Mutex<Idt> = Mutex::new(Idt::new());

pub fn host_idt_base() -> u64 {
    let idt = &*IDT.lock() as *const Idt;
    idt as u64
}

#[no_mangle]
pub extern "C" fn interrupt_dispatcher(state: &mut InterruptCpuState) {
    panic!("Unhandled interrupt {:x?}", state);
}

pub fn init_interrupt_handlers(cs: u16) {
    let mut idt = IDT.lock();
    for i in 0..20 {
        idt.set_entry(
            i,
            isr::ISR[i as usize] as u64,
            cs,
            IDT_ENTRY_FLAGS_RING_0 | IDT_ENTRY_FLAGS_PRESENT | IDT_ENTRY_FLAGS_INTERRUPT_GATE,
        );
    }
}

#[derive(Default)]
#[repr(packed)]
pub struct IdtDescriptor {
    pub limit: u16,
    pub base: u64,
}
