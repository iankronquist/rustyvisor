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

#[derive(Debug)]
#[repr(C)]
pub struct InterruptCpuState {
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