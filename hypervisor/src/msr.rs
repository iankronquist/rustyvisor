//! This module is used to work with Model Specific Registers.
//! A Model Specific Register, or MSR, is a hardware register used to configure
//! the hardware, learn about the current hardware configuration, or monitor the
//! state of the hardware.
//! MSRs may be unique per core, unique per NUMA node, or global for the whole
//! machine.

/// The values of various Model Specific Registers.
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

/// Represents the value of an Model specific register.
/// rdmsr returns the value with the high bits of the MSR in edx and the low bits in eax.
/// wrmsr recieves the value similarly.
pub struct MsrValuePair {
    pub edx: u32,
    pub eax: u32,
}

/// Read a model specific register as a pair of two values.
pub fn rdmsr(msr: Msr) -> MsrValuePair {
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
    MsrValuePair { edx, eax }
}

/// Read a model specific register as a single 64 bit value.
pub fn rdmsrl(msr: Msr) -> u64 {
    let pair = rdmsr(msr);
    (u64::from(pair.edx) << 32) | u64::from(pair.eax)
}

/// Write to a model specific register.
pub fn wrmsr(msr: Msr, pair: MsrValuePair) {
    unsafe {
        asm!(
        "wrmsr",
         in("eax")(pair.eax),
          in("edx")(pair.edx),
          in("ecx")(msr as u32)
        );
    }
}
