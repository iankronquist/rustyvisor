#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Msr {
    EFER = 0xc000_0080,
    Ia32FeatureControl = 0x0000_003a,
    Ia32DebugControl = 0x0000_01d9,
    Ia32VmxBasic = 0x0000_0480,
    Ia32VmxPinBasedControls = 0x0000_0481,
    Ia32VmxProcBasedControls = 0x0000_0482,
    Ia32VmxExitControls = 0x0000_0483,
    Ia32VmxEntryControls = 0x0000_0484,
    Ia32VmxMisc = 0x0000_0485,
    Ia32VmxCr0Fixed0 = 0x0000_0486,
    Ia32VmxCr0Fixed1 = 0x0000_0487,
    Ia32VmxCr4Fixed0 = 0x0000_0488,
    Ia32VmxCr4Fixed1 = 0x0000_0489,
    Ia32VmxVmcsEnum = 0x0000_048a,
    Ia32VmxProcBasedControls2 = 0x0000_048b,
    Ia32VmxEptVpidCap = 0x0000_048c,
    Ia32VmxTruePinBasedControls = 0x0000_048d,
    Ia32VmxTrueProcBasedControls = 0x0000_048e,
    Ia32VmxTrueExitControls = 0x0000_048f,
    Ia32VmxTrueEntryControls = 0x0000_0490,
    Ia32VmxVmFunc = 0x0000_0491,
}

pub fn rdmsr(msr: Msr) -> (u32, u32) {
    let edx: u32;
    let eax: u32;
    unsafe {
        asm!(
        "rdmsr",
         lateout("eax")(eax),
          lateout("edx")(edx),
          in("ecx")(msr as u32)
        );
    }
    (edx, eax)
}

pub fn rdmsrl(msr: Msr) -> u64 {
    let (edx, eax) = rdmsr(msr);
    (u64::from(edx) << 32) | u64::from(eax)
}

pub fn wrmsr(msr: Msr, eax: u32, edx: u32) {
    unsafe {
        asm!(
        "wrmsr",
         in("eax")(eax),
          in("edx")(edx),
          in("ecx")(msr as u32)
        );
    }
}
