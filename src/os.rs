#[repr(C)]
pub struct Translation {
    pub phys: u64,
    pub virt: u64,
}


#[repr(C)]
pub struct KernelData {
    pub heap_size: u64,
    pub translations_count: u64,
    pub heap: *mut u8,
    pub translations: *const Translation,
}
