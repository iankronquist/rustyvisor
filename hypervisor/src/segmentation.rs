const GDT_ENTRY_ACCESS_PRESENT: u8 = 1 << 7;

// Table 24-2 ch 24-4 vol 3c
const VMX_INFO_SEGMENT_UNUSABLE: u32 = 1 << 16;

#[derive(Default)]
pub struct UnpackedGdtEntry {
    base: u64,
    limit: u64,
    access_rights: u32,
    selector: u16,
}

pub fn unpack_gdt_entry(gdt: &[GdtEntry], selector: u16) -> UnpackedGdtEntry {
    let mut unpacked: UnpackedGdtEntry = Default::default();

    let index: usize = usize::from(selector) / core::mem::size_of::<GdtEntry>();
    if index == 0 {
        unpacked.access_rights |= VMX_INFO_SEGMENT_UNUSABLE;
        return unpacked;
    }

    unpacked.selector = selector;
    unpacked.limit = lsl(selector & !0x3);
    unpacked.base = u64::from(gdt[index].base_low);
    unpacked.base = (u64::from(gdt[index].base_high) << 24)
        | (u64::from(gdt[index].base_middle) << 16)
        | u64::from(gdt[index].base_low);

    unpacked.access_rights = u32::from(gdt[index].access);
    unpacked.access_rights |= (u32::from((gdt[index].granularity) & 0xf0) << 8);
    unpacked.access_rights &= 0xf0ff;
    if (gdt[index].access & GDT_ENTRY_ACCESS_PRESENT) == 0 {
        unpacked.access_rights |= VMX_INFO_SEGMENT_UNUSABLE;
    }

    unpacked
}

#[repr(packed)]
#[allow(dead_code)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
    base_highest: u32,
    reserved0: u32,
}

pub fn lsl(selector: u16) -> u64 {
    let limit: u64;
    let selector = u32::from(selector);
    unsafe {
        asm!("lsl {selector:e}, {limit}", limit = out(reg) limit, selector = in(reg) selector );
    }
    limit
}

pub fn sgdt(gdt_desc: *mut GdtDescriptor) {
    unsafe {
        asm!(
        "sgdt [{}]",
        in(reg) (gdt_desc)
        );
    }
}

pub fn get_current_gdt() -> &'static [GdtEntry] {
    let mut gdtr: GdtDescriptor = Default::default();
    sgdt(&mut gdtr);
    unsafe {
        core::slice::from_raw_parts(gdtr.base as *const GdtEntry, usize::from(gdtr.limit) + 1)
    }
}

#[derive(Default)]
#[repr(packed)]
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

#[repr(packed)]
pub struct Tss {
    reserved0: u32,
    stack0: u64,
    stack1: u64,
    stack2: u64,
    reserved1: u64,
    ist: [u64; 7],
    reserved2: u64,
    reserved3: u16,
    iomap_base: u16,
}