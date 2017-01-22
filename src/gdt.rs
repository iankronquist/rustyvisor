use vmx;
use cli;

#[repr(packed)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    flags: u8,
    more_flags: u8,
    base_high: u8,
}

const GDT: [GDTEntry; 3] = [
    GDTEntry{
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        flags: 0,
        more_flags: 0,
        base_high: 0,
    },
    GDTEntry{
        limit_low: 0xffff,
        base_low: 0,
        base_middle: 0,
        flags: 0b10101001,
        more_flags: 0b11110111,
        base_high: 0,
    },
    GDTEntry{
        limit_low: 0xffff,
        base_low: 0,
        base_middle: 0,
        flags: 0b00101001,
        more_flags: 0b11110111,
        base_high: 0,
    },
];

pub fn new_host_descriptor() -> vmx::CPUTableDescriptor {
    vmx::CPUTableDescriptor{ limit: 0xffff, base: GDT.as_ptr() as u64 }
}

pub fn test_load() {
    cli::ClearLocalInterruptsGuard::new();
    let gdt_desc = new_host_descriptor();
    let mut orig_gdt_desc: vmx::CPUTableDescriptor = Default::default();
    vmx::sgdt(&mut orig_gdt_desc);
    vmx::lgdt(&gdt_desc);
    vmx::lgdt(&orig_gdt_desc);
}
