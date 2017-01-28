use vmx;
use core::mem;

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

pub fn new_host_descriptor() -> vmx::CPUTableDescriptor {
    vmx::CPUTableDescriptor {
        limit: (mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: GDT.as_ptr() as u64,
    }
}

#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {

    use cli;
    use vmx;
    use super::new_host_descriptor;

    pub fn run() {
        test_load_and_restore_gdt();
    }

    fn test_load_and_restore_gdt() {
        cli::ClearLocalInterruptsGuard::new();
        let gdt_desc = new_host_descriptor();
        let mut orig_gdt_desc: vmx::CPUTableDescriptor = Default::default();
        vmx::sgdt(&mut orig_gdt_desc);
        vmx::lgdt(&gdt_desc);
        vmx::lgdt(&orig_gdt_desc);
    }
}
