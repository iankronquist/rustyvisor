/// Stores the state of the general purpose registers.
/// The order must be the same as the order of the pushes and pops in the assembly functions _host_entrypoint and _service_interrupt.
/// This structure does not include rsp since rsp is saved by iret in an interrupt and in the vmcs field GuestRsp by a vmexit.
#[derive(Debug)]
#[repr(C)]
pub struct GeneralPurposeRegisterState {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
}

impl GeneralPurposeRegisterState {
    /// Returns a mutable reference to a register in the GeneralPurposeRegisterState or None if that register is not in the state.
    /// Note that rsp is note in GeneralPurposeRegisterState.
    ///
    /// This order is related to the encoding of the mod rm byte and is widely used throughout various vmcs fields.
    /// For more information about the mod rm byte, see Vol 2. Section 2.1.3 particularly Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte".
    /// See also Vol 3. Table 27-3. "Exit Qualification for Control-Register Accesses" and Vol 3. Tables 27-9 through 27-14. and probably other places.
    pub fn by_mod_rm_index(&mut self, index: u64) -> Option<&mut u64> {
        match index {
            0x0 => Some(&mut self.rax),
            0x1 => Some(&mut self.rcx),
            0x2 => Some(&mut self.rdx),
            0x3 => Some(&mut self.rbx),
            0x4 => None, // For an interrupt rsp is pushed by the iret. For a vmexit, it's stored in the vmcs field GuestRsp.
            0x5 => Some(&mut self.rbp),
            0x6 => Some(&mut self.rsi),
            0x7 => Some(&mut self.rdi),
            0x8 => Some(&mut self.r8),
            0x9 => Some(&mut self.r9),
            0xa => Some(&mut self.r10),
            0xb => Some(&mut self.r11),
            0xc => Some(&mut self.r12),
            0xd => Some(&mut self.r13),
            0xe => Some(&mut self.r14),
            0xf => Some(&mut self.r15),
            _ => panic!("Illegal value"),
        }
    }
}

/// The register state when an interrupt is taken.
/// This order must be the same as the order of the pushes and pops in the assembly function _service_interrupt.
/// The order of the general purpose registers and gs, fs, es, and ds is arbitrary. The registers rip, cs, rflags, ss, and rsp are pushed by the interrupt, and their order is fixed by hardware.
#[derive(Debug)]
#[repr(C)]
pub struct InterruptRegisterState {
    gs: u64,
    fs: u64,
    es: u64,
    ds: u64,
    registers: GeneralPurposeRegisterState,
    interrupt_number: u64,
    error_code: u64,
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}
