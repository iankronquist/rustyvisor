use ::log::{error, info, log};

use core::{mem, ptr};

#[repr(u32)]
pub enum MSR {
    EFER = 0xc000_0080,
    Ia32FeatureControl = 0x0000_003a,
    Ia32VmxBasic = 0x0000_0480,
    Ia32VmxPinBasedCtls = 0x0000_0481,
    Ia32VmxProcBasedCtls = 0x0000_0482,
    Ia32VmxExitCtls = 0x0000_0483,
    Ia32VmxEntryCtls = 0x0000_0484,
    Ia32VmxMisc = 0x0000_0485,
    Ia32VmxCr0Fixed0 = 0x0000_0486,
    Ia32VmxCr0Fixed1 = 0x0000_0487,
    Ia32VmxCr4Fixed0 = 0x0000_0488,
    Ia32VmxCr4Fixed1 = 0x0000_0489,
    Ia32VmxVmcsEnum = 0x0000_048a,
    Ia32VmxProcBasedCtls2 = 0x0000_048b,
    Ia32VmxEptVpidCap = 0x0000_048c,
    Ia32VmxTruePinBasedCtls = 0x0000_048d,
    Ia32VmxTrueProcBasedCtls = 0x0000_048e,
    Ia32VmxTrueExitCtls = 0x0000_048f,
    Ia32VmxTrueEntryCtls = 0x0000_0490,
    Ia32VmxVmFunc = 0x0000_0491,
}

fn vm_instruction_error_number_message(n: u64) -> &'static str {
    match n {
        1 => "VMALL executed in VMX root operation",
        2 => "VMCLEAR with invalid physical address",
        3 => "VMCLEAR with VMXON pointer",
        4 => "VMLAUNCH with non-clear VMCS",
        5 => "VMRESUME with non-launched VMCS",
        6 => "VMRESUME after VMXOFF (VMXOFF and VMXON between VMLAUNCH and VMRESUME)",
        7 => "VM entry with invalid control field(s)",
        8 => "VM entry with invalid host-state field(s)",
        9 => "VMPTRLD with invalid physical address",
        10 => "VMPTRLD with VMXON pointer",
        11 => "VMPTRLD with incorrect VMCS revision identifier",
        12 => "VMREAD/VMWRITE from/to unsupported VMCS component",
        13 => "VMWRITE to read-only VMCS component",
        15 => "VMXON executed in VMX root operation",
        16 => "VM entry with invalid executive-VMCS pointer",
        17 => "VM entry with non-launched executive VMCS",
        18 => {
            "VM entry with executive-VMCS pointer not VMXON pointer (when attempting to deactivate \
             the dual-monitor treatment of"
        }
        19 => {
            "VMCALL with non-clear VMCS (when attempting to activate the dual-monitor treatment of \
             SMIs and SMM)"
        }
        20 => "VMCALL with invalid VM-exit control fields",
        22 => {
            "VMCALL with incorrect MSEG revision identifier (when attempting to activate the \
             dual-monitor treatment of SMIs
            and SMM)"
        }
        23 => "VMXOFF under dual-monitor treatment of SMIs and SMM",
        24 => {
            "VMCALL with invalid SMM-monitor features (when attempting to activate the \
             dual-monitor treatment of SMIs and SMM)"
        }
        25 => {
            "VM entry with invalid VM-execution control fields in executive VMCS (when attempting \
             to return from SMM)"
        }
        26 => "VM entry with events blocked by MOV SS.",
        28 => "Invalid operand to INVEPT/INVVPID.",
        _ => "Unknown VM instruction error number.",
    }
}

const IA32_FEATURE_CONTROL_LOCK_BIT: u32 = (1 << 0);
const IA32_FEATURE_CONTROL_VMX_ENABLED_OUTSIDE_SMX_BIT: u32 = (1 << 2);

#[repr(u32)]
pub enum CPUIDLeaf {
    ProcessorInfoAndFeatures = 1,
}

#[repr(u32)]
pub enum CPUIDLeafProcessorInfoAndFeaturesECXBits {
    VMXAvailable = 1 << 5,
    HypervisorPresent = 1 << 31,
}

#[repr(u64)]
pub enum VMCSField {
    VirtualProcessorID = 0x0000_0000,
    PostedIntrNV = 0x0000_0002,
    GuestESSelector = 0x0000_0800,
    GuestCSSelector = 0x0000_0802,
    GuestSSSelector = 0x0000_0804,
    GuestDSSelector = 0x0000_0806,
    GuestFSelector = 0x0000_0808,
    GuestGSSelector = 0x0000_080a,
    GuestLDTRSelector = 0x0000_080c,
    GuestTrSelector = 0x0000_080e,
    GuestIntrStatus = 0x0000_0810,
    GuestPmlIndex = 0x0000_0812,
    HostESSelector = 0x0000_0c00,
    HostCSSelector = 0x0000_0c02,
    HostSSSelector = 0x0000_0c04,
    HostDSSelector = 0x0000_0c06,
    HostFSelector = 0x0000_0c08,
    HostGSSelector = 0x0000_0c0a,
    HostTrSelector = 0x0000_0c0c,
    IOBitmapA = 0x0000_2000,
    IOBitmapAHigh = 0x0000_2001,
    IOBitmapB = 0x0000_2002,
    IOBitmapBHigh = 0x0000_2003,
    MSRBitmap = 0x0000_2004,
    MSRBitmapHigh = 0x0000_2005,
    VMExitMSRStoreAddr = 0x0000_2006,
    VMExitMSRStoreAddrHigh = 0x0000_2007,
    VMExitMSRLoadAddr = 0x0000_2008,
    VMExitMSRLoadAddrHigh = 0x0000_2009,
    VMEntryMSRLoadAddr = 0x0000_200a,
    VMEntryMSRLoadAddrHigh = 0x0000_200b,
    PMLAddress = 0x0000_200e,
    PMLAddressHigh = 0x0000_200f,
    TSCOffset = 0x0000_2010,
    TSCOffsetHigh = 0x0000_2011,
    VirtualApicPageAddr = 0x0000_2012,
    VirtualApicPageAddrHigh = 0x0000_2013,
    APICAccessAddr = 0x0000_2014,
    APICAccessAddrHigh = 0x0000_2015,
    PostedIntrDescAddr = 0x0000_2016,
    PostedIntrDescAddrHigh = 0x0000_2017,
    EPTPointer = 0x0000_201a,
    EPTPointerHigh = 0x0000_201b,
    EOIExitBitmap0 = 0x0000_201c,
    EOIExitBitmap0High = 0x0000_201d,
    EOIExitBitmap1 = 0x0000_201e,
    EOIExitBitmap1High = 0x0000_201f,
    EOIExitBitmap2 = 0x0000_2020,
    EOIExitBitmap2High = 0x0000_2021,
    EOIExitBitmap3 = 0x0000_2022,
    EOIExitBitmap3High = 0x0000_2023,
    VMReadBitmap = 0x0000_2026,
    VMWriteBitmap = 0x0000_2028,
    XSSExitBitmap = 0x0000_202c,
    XSSExitBitmapHigh = 0x0000_202d,
    TSXMultiplier = 0x0000_2032,
    TSXMultiplierHigh = 0x0000_2033,
    GuestPhysicalAddress = 0x0000_2400,
    GuestPhysicalAddressHigh = 0x0000_2401,
    VMcsLinkPointer = 0x0000_2800,
    VMcsLinkPointerHigh = 0x0000_2801,
    GuestIA32Debugctl = 0x0000_2802,
    GuestIA32DebugctlHigh = 0x0000_2803,
    GuestIA32Pat = 0x0000_2804,
    GuestIA32PatHigh = 0x0000_2805,
    GuestIA32Efer = 0x0000_2806,
    GuestIA32EferHigh = 0x0000_2807,
    GuestIA32PerfGlobalCtrl = 0x0000_2808,
    GuestIA32PerfGlobalCtrlHigh = 0x0000_2809,
    GuestPDPtr0 = 0x0000_280a,
    GuestPDPtr0High = 0x0000_280b,
    GuestPDPtr1 = 0x0000_280c,
    GuestPDPtr1High = 0x0000_280d,
    GuestPDPtr2 = 0x0000_280e,
    GuestPDPtr2High = 0x0000_280f,
    GuestPDPtr3 = 0x0000_2810,
    GuestPDPtr3High = 0x0000_2811,
    GuestBndcfgs = 0x0000_2812,
    GuestBndcfgsHigh = 0x0000_2813,
    HostIA32Pat = 0x0000_2c00,
    HostIA32PatHigh = 0x0000_2c01,
    HostIA32Efer = 0x0000_2c02,
    HostIA32EferHigh = 0x0000_2c03,
    HostIA32PerfGlobalCtrl = 0x0000_2c04,
    HostIA32PerfGlobalCtrlHigh = 0x0000_2c05,
    PinBasedVMExecControl = 0x0000_4000,
    CPUBasedVMExecControl = 0x0000_4002,
    ExceptionBitmap = 0x0000_4004,
    PageFaultErrorCodeMask = 0x0000_4006,
    PageFaultErrorCodeMatch = 0x0000_4008,
    CR3TargetCount = 0x0000_400a,
    VMExitControls = 0x0000_400c,
    VMExitMsrStoreCount = 0x0000_400e,
    VMExitMsrLoadCount = 0x0000_4010,
    VMEntryControls = 0x0000_4012,
    VMEntryMsrLoadCount = 0x0000_4014,
    VMEntryIntrInfoField = 0x0000_4016,
    VMEntryExceptionErrorCode = 0x0000_4018,
    VMEntryInstructionLen = 0x0000_401a,
    TPRThreshold = 0x0000_401c,
    SecondaryVMExecControl = 0x0000_401e,
    PLEGap = 0x0000_4020,
    PLEWindow = 0x0000_4022,
    VMInstructionError = 0x0000_4400,
    VMExitReason = 0x0000_4402,
    VMExitIntrInfo = 0x0000_4404,
    VMExitIntrErrorCode = 0x0000_4406,
    IdtVectoringInfoField = 0x0000_4408,
    IdtVectoringErrorCode = 0x0000_440a,
    VMExitInstructionLen = 0x0000_440c,
    VMXInstructionInfo = 0x0000_440e,
    GuestESLimit = 0x0000_4800,
    GuestCSLimit = 0x0000_4802,
    GuestSSLimit = 0x0000_4804,
    GuestDSLimit = 0x0000_4806,
    GuestFsLimit = 0x0000_4808,
    GuestGSLimit = 0x0000_480a,
    GuestLDTRLimit = 0x0000_480c,
    GuestTrLimit = 0x0000_480e,
    GuestGDTRLimit = 0x0000_4810,
    GuestIDTRLimit = 0x0000_4812,
    GuestESArBytes = 0x0000_4814,
    GuestCSArBytes = 0x0000_4816,
    GuestSSArBytes = 0x0000_4818,
    GuestDSArBytes = 0x0000_481a,
    GuestFSArBytes = 0x0000_481c,
    GuestGSArBytes = 0x0000_481e,
    GuestLDTRArBytes = 0x0000_4820,
    GuestTRArBytes = 0x0000_4822,
    GuestInterruptibilityInfo = 0x0000_4824,
    GuestActivityState = 0x0000_4826,
    GuestSysenterCS = 0x0000_482a,
    VMXPreemptionTimerValue = 0x0000_482e,
    HostIA32SysenterCS = 0x0000_4c00,
    CR0GuestHostMask = 0x0000_6000,
    CR4GuestHostMask = 0x0000_6002,
    CR0ReadShadow = 0x0000_6004,
    CR4ReadShadow = 0x0000_6006,
    CR3TargetValue0 = 0x0000_6008,
    CR3TargetValue1 = 0x0000_600a,
    CR3TargetValue2 = 0x0000_600c,
    CR3TargetValue3 = 0x0000_600e,
    ExitQualification = 0x0000_6400,
    GuestLinearAddress = 0x0000_640a,
    GuestCR0 = 0x0000_6800,
    GuestCR3 = 0x0000_6802,
    GuestCR4 = 0x0000_6804,
    GuestESBase = 0x0000_6806,
    GuestCSBase = 0x0000_6808,
    GuestSSBase = 0x0000_680a,
    GuestDSBase = 0x0000_680c,
    GuestFsBase = 0x0000_680e,
    GuestGSBase = 0x0000_6810,
    GuestLDTRBase = 0x0000_6812,
    GuestTRBase = 0x0000_6814,
    GuestGDTRBase = 0x0000_6816,
    GuestIDTRBase = 0x0000_6818,
    GuestDR7 = 0x0000_681a,
    GuestRSP = 0x0000_681c,
    GuestRIP = 0x0000_681e,
    GuestRFlags = 0x0000_6820,
    GuestPendingDbgExceptions = 0x0000_6822,
    GuestSysenterESP = 0x0000_6824,
    GuestSysenterEIP = 0x0000_6826,
    HostCR0 = 0x0000_6c00,
    HostCR3 = 0x0000_6c02,
    HostCR4 = 0x0000_6c04,
    HostFSBase = 0x0000_6c06,
    HostGSBase = 0x0000_6c08,
    HostTRBase = 0x0000_6c0a,
    HostGDTRBase = 0x0000_6c0c,
    HostIDTRBase = 0x0000_6c0e,
    HostIA32SysenterESP = 0x0000_6c10,
    HostIA32SysenterEIP = 0x0000_6c12,
    HostRSP = 0x0000_6c14,
    HostRIP = 0x0000_6c16,
}

pub const fn is_page_aligned(n: u64) -> bool {
    n.trailing_zeros() >= 12
}

pub fn cpuid(mut eax: u32) -> (u32, u32, u32, u32) {
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    unsafe {
        asm!("cpuid"
          : "+{eax}"(eax), "={ebx}"(ebx), "={ecx}"(ecx), "={edx}"(edx)
          :
          :
        )
    };

    (eax, ebx, ecx, edx)
}

pub fn vmxon(addr: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmxon $1; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret)
         : "m"(addr)
         :
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn vmxoff() {
    unsafe {
        asm!(
        "vmxoff"
        :
        :
        :
        );
    }
}

pub fn vmread(field: VMCSField) -> Result<u64, u32> {
    let ret: u32;
    let val: u64;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmread $2, $1; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret) "=r"(val)
         : "r"(field)
         :
        );
    }
    if ret == 0 {
        Ok(val)
    } else {
        Err(ret)
    }
}

pub fn vmwrite(field: VMCSField, val: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmread $2, $1; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret)
         : "r"(field) "r"(val)
         :
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn rdmsr(msr: MSR) -> (u32, u32) {
    let edx: u32;
    let eax: u32;
    unsafe {
        asm!(
        "rdmsr"
         : "={eax}"(eax) "={edx}"(edx)
         : "{ecx}"(msr)
         :
        );
    }
    (edx, eax)
}

pub fn rdmsrl(msr: MSR) -> u64 {
    let edx: u32;
    let eax: u32;
    unsafe {
        asm!(
        "rdmsr"
         : "={eax}"(eax) "={edx}"(edx)
         : "{ecx}"(msr)
         :
        );
    }
    (u64::from(edx) << 32) | u64::from(eax)
}

pub fn wrmsr(msr: MSR, eax: u32, edx: u32) {
    unsafe {
        asm!(
        "mov $1, %ecx; \
         wrmsr"
         :
         : "{ecx}"(msr) "{eax}"(eax) "{edx}"(edx)
         :
        );
    }
}

pub fn vmptrld(vmcs_phys: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmptrld $1; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret)
         : "m"(vmcs_phys)
         :
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn vmclear(vmcs_phys: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmclear $1; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret)
         : "m"(vmcs_phys)
         :
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn invvpid(vmcs_phys: u64) {
    unsafe {
        asm!(
        "vmclear $0;"
         :
         : "m"(vmcs_phys)
         :
        );
    }
}

pub fn vmptrst() -> Result<u64, u32> {
    let ret: u32;
    let mut vmcs_phys: u64 = 0;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmptrst $1; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret) "=*m"(&mut vmcs_phys)
         :
         :
        );
    }
    if ret == 0 {
        Ok(vmcs_phys)
    } else {
        Err(ret)
    }
}

pub fn vmlaunch() -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmlaunch; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret)
         :
         :
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn vmresume() -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor %eax, %eax; \
         vmresume; \
         setc %ah; \
         setz %al;"
         : "={eax}"(ret)
         :
         :
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn read_cs() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov %cs, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_ss() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov %ss, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_ds() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov %ds, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_es() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov %es, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_fs() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov %fs, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_gs() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov %gs, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn write_cs(val: u16) {
    unsafe {
        asm!(
        "mov $0, %cs"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_ss(val: u16) {
    unsafe {
        asm!(
        "mov $0, %ss"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_ds(val: u16) {
    unsafe {
        asm!(
        "mov $0, %ds"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_es(val: u16) {
    unsafe {
        asm!(
        "mov $0, %es"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_fs(val: u16) {
    unsafe {
        asm!(
        "mov $0, %fs"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_gs(val: u16) {
    unsafe {
        asm!(
        "mov $0, %gs"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_cr0(val: u64) {
    unsafe {
        asm!(
        "mov $0, %cr0"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_cr3(val: u64) {
    unsafe {
        asm!(
        "mov $0, %cr3"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_cr4(val: u64) {
    unsafe {
        asm!(
        "mov $0, %cr4"
        :
        : "r"(val)
        :
        );
    }
}

pub fn write_db7(val: u64) {
    unsafe {
        asm!(
        "mov $0, %db7"
        :
        : "r"(val)
        :
        );
    }
}

pub fn read_cr0() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "mov %cr0, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_cr3() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "mov %cr3, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_cr4() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "mov %cr4, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_db7() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "mov %db7, $0"
        : "=r"(ret)
        :
        :
        );
    }
    ret
}

pub fn read_flags() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "pushf; pop $0"
        : "=r"(ret)
        :
        : "memory"
        );
    }
    ret
}

fn vmx_available() -> bool {
    let (_eax, _ebx, ecx, _edx) = cpuid(CPUIDLeaf::ProcessorInfoAndFeatures as u32);
    ecx & (CPUIDLeafProcessorInfoAndFeaturesECXBits::VMXAvailable as u32) != 0
}

// FIXME: Memoize
fn get_vmcs_revision_identifier() -> u32 {
    let (_high_bits, vmcs_revision_identifier) = rdmsr(MSR::Ia32VmxBasic);
    assert!((vmcs_revision_identifier & (1 << 31)) == 0);
    vmcs_revision_identifier
}

fn set_cr0_bits() {
    let fixed0 = rdmsrl(MSR::Ia32VmxCr0Fixed0);
    let fixed1 = rdmsrl(MSR::Ia32VmxCr0Fixed1);
    let mut cr0 = read_cr0();
    cr0 |= fixed0;
    cr0 &= fixed1;
    write_cr0(cr0);
}

fn set_cr4_bits() {
    let fixed0 = rdmsrl(MSR::Ia32VmxCr4Fixed0);
    let fixed1 = rdmsrl(MSR::Ia32VmxCr4Fixed1);
    let mut cr4 = read_cr4();
    cr4 |= fixed0;
    cr4 &= fixed1;
    write_cr4(cr4);
}

fn set_lock_bit() -> Result<(), ()> {
    let (_high, low) = rdmsr(MSR::Ia32FeatureControl);
    if (low & IA32_FEATURE_CONTROL_LOCK_BIT) == 0 {
        wrmsr(
            MSR::Ia32FeatureControl,
            _high,
            low | IA32_FEATURE_CONTROL_VMX_ENABLED_OUTSIDE_SMX_BIT | IA32_FEATURE_CONTROL_LOCK_BIT,
        );
        Ok(())
    } else if (low & IA32_FEATURE_CONTROL_VMX_ENABLED_OUTSIDE_SMX_BIT) == 0 {
        Err(())
    } else {
        Ok(())
    }
}

#[warn(clippy::cast_ptr_alignment)]
fn prepare_vmx_memory_region(vmx_region: *mut u8, vmx_region_size: usize) {
    assert!(!vmx_region.is_null());
    assert!(vmx_region_size <= 0x1000);
    assert!(vmx_region_size > mem::size_of::<u32>());

    unsafe {
        ptr::write_bytes(vmx_region, 0, vmx_region_size);
        ptr::write(vmx_region as *mut u32, get_vmcs_revision_identifier());
    }
}

pub fn enable(
    vmxon_region: *mut u8,
    vmxon_region_phys: u64,
    vmxon_region_size: usize,
) -> Result<(), ()> {
    assert!(is_page_aligned(vmxon_region as u64));
    assert!(is_page_aligned(vmxon_region_phys));

    if vmxon_region.is_null() {
        error!("Bad VMX on region");
        return Err(());
    }

    if !vmx_available() {
        error!("VMX unavailable");
        return Err(());
    }

    set_lock_bit().or_else(|_| {
        error!("Lock bit not set");
        Err(())
    })?;

    set_cr0_bits();
    set_cr4_bits();

    prepare_vmx_memory_region(vmxon_region, vmxon_region_size);

    let result = vmxon(vmxon_region_phys);
    // FIXME: Fix error types
    if result == Ok(()) {
        info!("vmxon succeeded");
        Ok(())
    } else {
        error!("vmxon failed");
        Err(())
    }
}

fn vmcs_initialize_host_state() {}

fn vmcs_initialize_guest_state() {}

fn vmcs_initialize_vm_control_values() {
    // Simon, this is your place to ☆shine☆!
}

pub fn disable() {
    vmxoff();
    info!("vmxoff");
}

pub fn load_vm(vmcs: *mut u8, vmcs_phys: u64, vmcs_size: usize) -> Result<(), ()> {
    assert!(is_page_aligned(vmcs as u64));
    assert!(is_page_aligned(vmcs_phys));

    prepare_vmx_memory_region(vmcs, vmcs_size);

    vmclear(vmcs_phys).or_else(|_| Err(()))?;
    vmptrld(vmcs_phys).or_else(|_| Err(()))?;

    vmcs_initialize_host_state();
    vmcs_initialize_guest_state();
    vmcs_initialize_vm_control_values();

    vmlaunch().or_else(|_| {
        match vmread(VMCSField::VMInstructionError) {
            Ok(vm_instruction_error_number) => error!(
                "Failed to launch VM because {} ({})",
                vm_instruction_error_number_message(vm_instruction_error_number),
                vm_instruction_error_number
            ),
            Err(e) => error!("VMLaunch failed with {}", e),
        }
        Err(())
    })?;

    Ok(())
}

pub fn unload_vm() {
    if let Ok(vmcs_phys) = vmptrst() {
        if let Err(code) = vmclear(vmcs_phys) {
            error!("vmclear failed with error code {}", code);
        }
    }
}
