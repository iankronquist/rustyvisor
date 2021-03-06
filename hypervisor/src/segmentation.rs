//! Defines structures and code for working with x86 segments.

use log::trace;
use x86;
const GDT_ENTRY_ACCESS_PRESENT: u8 = 1 << 7;

// Table 24-2 ch 24-4 vol 3c
const VMX_INFO_SEGMENT_UNUSABLE: u32 = 1 << 16;

#[derive(Default, Debug)]
pub struct UnpackedGdtEntry {
    pub base: u64,
    pub limit: u64,
    pub access_rights: u32,
    pub selector: u16,
}

impl UnpackedGdtEntry {
    pub fn is_usable(&self) -> bool {
        self.access_rights != VMX_INFO_SEGMENT_UNUSABLE
    }
}

pub fn unpack_gdt_entry(gdt: &[GdtEntry], selector: u16) -> UnpackedGdtEntry {
    let mut unpacked: UnpackedGdtEntry = Default::default();

    let index: usize = usize::from(selector) / core::mem::size_of::<GdtEntry>();
    if index == 0 {
        unpacked.access_rights |= VMX_INFO_SEGMENT_UNUSABLE;
        trace!("Unpacked {:x?}", unpacked);
        return unpacked;
    }

    unpacked.selector = selector;
    unpacked.limit =
        u64::from(gdt[index].limit_low) | ((u64::from(gdt[index].granularity) & 0x0f) << 16);
    unpacked.base = u64::from(gdt[index].base_low);
    unpacked.base = (u64::from(gdt[index].base_high) << 24)
        | (u64::from(gdt[index].base_middle) << 16)
        | u64::from(gdt[index].base_low);

    unpacked.access_rights = u32::from(gdt[index].access);
    unpacked.access_rights |= u32::from((gdt[index].granularity) & 0xf0) << 8;
    unpacked.access_rights &= 0xf0ff;
    if (gdt[index].access & GDT_ENTRY_ACCESS_PRESENT) == 0 {
        unpacked.access_rights |= VMX_INFO_SEGMENT_UNUSABLE;
    }

    trace!("Gdt entry {:x?}", gdt[index]);
    trace!("Gdt entry unpacked {:x?}", unpacked);
    unpacked
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
#[repr(packed)]
pub struct GdtEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
}

#[allow(unused)]
#[repr(packed)]
pub struct GdtEntry64 {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
    pub base_highest: u32,
    pub reserved0: u32,
}

pub fn get_current_gdt() -> &'static [GdtEntry] {
    let mut gdtr: x86::dtables::DescriptorTablePointer<u64> = Default::default();
    unsafe {
        x86::dtables::sgdt(&mut gdtr);
    }
    trace!("Gdtr is {:x?}", gdtr);
    let bytes = usize::from(gdtr.limit) + 1;
    unsafe {
        core::slice::from_raw_parts(
            gdtr.base as *const GdtEntry,
            bytes / core::mem::size_of::<GdtEntry>(),
        )
    }
}

#[derive(Default)]
#[repr(packed)]
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

#[allow(unused)]
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
