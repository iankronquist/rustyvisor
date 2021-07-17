//! Defines structures and code for working with x86 segments.

use log::trace;
use x86;
const GDT_ENTRY_ACCESS_PRESENT: u8 = 1 << 7;

// See Intel manual Table 24-2 ch 24-4 vol 3c
const VMX_INFO_SEGMENT_UNUSABLE: u32 = 1 << 16;

/// GDT entries are packed in a complicated way meant to be backwards
/// compatible since the days of the i286. This represents the component parts
///of a GDT entry unpacked into a format we can feed into various host and
/// guest VMCS entries.
#[derive(Default, Debug)]
pub struct UnpackedGdtEntry {
    /// The base of the segment.
    pub base: u64,
    /// The limit of the segment.
    pub limit: u64,
    /// The access rights of the segment.
    pub access_rights: u32,
    /// The segment selector.
    pub selector: u16,
}

impl UnpackedGdtEntry {
    /// Checks to see if the GDT entry is usable.
    /// If the UEFI guest TR is unusable, a new one must be created since we
    /// need a host TSS to launch a VM.
    pub fn is_usable(&self) -> bool {
        self.access_rights != VMX_INFO_SEGMENT_UNUSABLE
    }
}

/// Given a global descriptor table, and a selector which indexes into the
/// table, unpack the corresponding GDT entry into an UnpackedGdtEntry.
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

/// 32 bit GDT entry.
/// The layout of this structure is determined by hardware.
/// For more information see the Intel manual, Volume 3, Chapter 5
/// ("Protection"), Section 5.2 "Fields and Flags Used for Segment-Level and
/// Page-Level Protection".
/// See also the OS Dev wiki page on the [GDT](https://wiki.osdev.org/GDT) and
/// the accompanying [tutorial](https://wiki.osdev.org/GDT_Tutorial).
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
#[repr(packed)]
pub struct GdtEntry {
    /// Low 16 bits of the segment limit.
    pub limit_low: u16,
    /// Low 16 bits of the segment base.
    pub base_low: u16,
    /// Middle 8 bits of the segment base.
    pub base_middle: u8,
    /// Various flags used to set segment type and access rights.
    pub access: u8,
    /// The low 4 bits are part of the limit. The high 4 bits are the
    /// granularity of the segment and the size.
    pub granularity: u8,
    /// High 8 bits of the segment base.
    pub base_high: u8,
}

/// 64 bit GDT entry.
/// The layout of this structure is determined by hardware.
/// For more information see the Intel manual, Volume 3, Chapter 3
/// ("Protected-Mode Memory Management"), "Section 3.5.2 Segment Descriptor
/// Tables in IA-32e Mode".
/// See also Volume 3, Chapter 7 ("Task Mangement"), Section 7.2.3 "TSS
/// Descriptor in 64-bit mode".
/// See also the OS Dev wiki page on the [GDT](https://wiki.osdev.org/GDT) and
/// the accompanying [tutorial](https://wiki.osdev.org/GDT_Tutorial).
#[allow(unused)]
#[repr(packed)]
pub struct GdtEntry64 {
    /// Low 16 bits of the segment limit.
    pub limit_low: u16,
    /// Low 16 bits of the segment base.
    pub base_low: u16,
    /// Middle 8 bits of the segment base.
    pub base_middle: u8,
    /// Various flags used to set segment type and access rights.
    pub access: u8,
    /// The low 4 bits are part of the limit. The high 4 bits are the
    /// granularity of the segment and the size.
    pub granularity: u8,
    /// Higher 8 bits of the segment base.
    pub base_high: u8,
    /// Highest 32 bits of the segment base.
    pub base_highest: u32,
    /// Reserved 0.
    pub reserved0: u32,
}

/// Get a reference to the processor's current GDT.
/// Note that we can't
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

/// Global Descriptor Table Register.
/// Describes the size and base of the GDT.
/// For more information see the Intel manual, Volume 3, Chapter 2 ("System
/// Architecture Overview"), Section 2.4.1 "Global Descriptor Table Register
/// GDTR)", and the accompanying figure.
/// See also the OS Dev wiki page on the [GDT](https://wiki.osdev.org/GDT) and
/// the accompanying [tutorial](https://wiki.osdev.org/GDT_Tutorial).
#[derive(Default)]
#[repr(packed)]
pub struct GdtDescriptor {
    /// The limit of the Global Descriptor Table.
    /// That is the size of the GDT minus one.
    pub limit: u16,
    /// The base virtual address of the GDT.
    pub base: u64,
}

/// The Task Struct Segment.
/// This is used for hardware task switching on 32 bit x86 and for holding
/// interrupt stack bases on 64 bit x86.
/// For more information see the Intel manual, Volume 3, Chapter 7 ("Task
/// Management"), Figure 7-11 "64-Bit TSS Format".
/// and the OS Dev wiki page on the
/// [TSS](https://wiki.osdev.org/Task_State_Segment).
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
