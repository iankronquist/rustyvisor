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

#[derive(Default)]
#[repr(packed)]
pub struct IDTDescriptor {
    pub limit: u16,
    pub base: u64,
}
