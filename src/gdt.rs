use core::mem;
use cli;

#[repr(packed)]
#[allow(dead_code)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
    base_highest: u32,
    reserved0: u32,
}

pub fn lgdt(gdt_desc: *const GDTDescriptor) {
    unsafe {
        asm!(
            "lgdt ($0)"
            :
            : "r"(gdt_desc)
            :
            );
    }
}

pub fn sgdt(gdt_desc: *mut GDTDescriptor) {
    unsafe {
        asm!(
            "sgdt ($0)"
            :
            : "r"(gdt_desc)
            :
            );
    }
}

const GDT: [GDTEntry; 3] = [GDTEntry {
                                limit_low: 0,
                                base_low: 0,
                                base_middle: 0,
                                access: 0,
                                granularity: 0,
                                base_high: 0,
                                base_highest: 0,
                                reserved0: 0,
                            },
                            GDTEntry {
                                limit_low: 0xffff,
                                base_low: 0,
                                base_middle: 0,
                                access: 0b10101001,
                                granularity: 0b11110111,
                                base_high: 0,
                                base_highest: 0,
                                reserved0: 0,
                            },
                            GDTEntry {
                                limit_low: 0xffff,
                                base_low: 0,
                                base_middle: 0,
                                access: 0b00101001,
                                granularity: 0b11110111,
                                base_high: 0,
                                base_highest: 0,
                                reserved0: 0,
                            }];

#[derive(Default)]
#[repr(packed)]
pub struct GDTDescriptor {
    pub limit: u16,
    pub base: u64,
}

impl GDTDescriptor {
    pub fn new() -> cli::ClearLocalInterruptsGuard<GDTDescriptor> {
        cli::ClearLocalInterruptsGuard::new(GDTDescriptor {
            limit: (mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
            base: GDT.as_ptr() as u64,
        })
    }

    pub fn from_cpu() -> cli::ClearLocalInterruptsGuard<GDTDescriptor> {
        let mut current_gdt_ptr: GDTDescriptor = Default::default();
        sgdt(&mut current_gdt_ptr);
        cli::ClearLocalInterruptsGuard::new(current_gdt_ptr)
    }

    pub fn load(&self) {
        lgdt(self);
    }
}

#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {

    use super::GDTDescriptor;

    pub fn run() {
        test_load_and_restore_gdt();
    }

    fn test_load_and_restore_gdt() {
        let orig_gdt_desc = GDTDescriptor::from_cpu();
        let gdt_desc = GDTDescriptor::new();
        gdt_desc.load();
        orig_gdt_desc.load();
    }
}
