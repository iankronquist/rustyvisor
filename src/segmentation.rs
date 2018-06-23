use core::mem;

#[repr(packed)]
#[allow(dead_code)]
pub struct GDTEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
    pub base_highest: u32,
    pub reserved0: u32,
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

const LIMIT_LOW: u16 = !0;

const LIMIT_HIGH_NIBBLE: u8 = 0x00ff;
const LONG_MODE: u8 = 1 << 5;
const GRANULARITY_4K: u8 = 1 << 7;

const ACCESS_WRITABLE: u8 = 1 << 1;
const ACCESS_CODE: u8 = 1 << 3;
const ACCESS_SET_HIGH: u8 = 1 << 4;
const ACCESS_PRESENT: u8 = 1 << 7;



const GDT: [GDTEntry; 3] = [
    GDTEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
        base_highest: 0,
        reserved0: 0,
    },
    // Ring 0 data segment.
    GDTEntry {
        limit_low: LIMIT_LOW,
        base_low: 0,
        base_middle: 0,
        access: ACCESS_PRESENT | ACCESS_SET_HIGH | ACCESS_WRITABLE,
        granularity: GRANULARITY_4K | LIMIT_HIGH_NIBBLE,
        base_high: 0,
        base_highest: 0,
        reserved0: 0,
    },
    // Ring 0 code segment.
    GDTEntry {
        limit_low: LIMIT_LOW,
        base_low: 0,
        base_middle: 0,
        access: ACCESS_PRESENT | ACCESS_SET_HIGH | ACCESS_CODE | ACCESS_WRITABLE,
        granularity: GRANULARITY_4K | LONG_MODE | LIMIT_HIGH_NIBBLE,
        base_high: 0,
        base_highest: 0,
        reserved0: 0,
    },
];

#[derive(Default)]
#[repr(packed)]
pub struct GDTDescriptor {
    pub limit: u16,
    pub base: u64,
}

impl<'a> GDTDescriptor {
    pub fn new() -> GDTDescriptor {
        GDTDescriptor {
            limit: (mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
            base: GDT.as_ptr() as u64,
        }
    }

    pub fn from_cpu() -> GDTDescriptor {
        let mut current_gdt_ptr: GDTDescriptor = Default::default();
        sgdt(&mut current_gdt_ptr);
        current_gdt_ptr
    }

    pub fn load(&self) {
        lgdt(self);
    }
}

#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {

    use super::GDTDescriptor;

    pub fn run() {
        info!("Executing GDT tests...");
        test_load_and_restore_gdt();
        info!("GDT tests succeeded");
    }

    fn test_load_and_restore_gdt() {
        let orig_gdt_desc = GDTDescriptor::from_cpu();
        let gdt_desc = GDTDescriptor::new();
        gdt_desc.load();
        orig_gdt_desc.load();
    }
}
