use core::mem;
use interrupts;
use segmentation;

#[repr(u32)]
pub enum MSR {
    EFER = 0xc0000080,
    Ia32FeatureControl = 0x0000003a,
    Ia32VmxBasic = 0x00000480,
    Ia32VmxPinBasedCtls = 0x00000481,
    Ia32VmxProcBasedCtls = 0x00000482,
    Ia32VmxExitCtls = 0x00000483,
    Ia32VmxEntryCtls = 0x00000484,
    Ia32VmxMisc = 0x00000485,
    Ia32VmxCr0Fixed0 = 0x00000486,
    Ia32VmxCr0Fixed1 = 0x00000487,
    Ia32VmxCr4Fixed0 = 0x00000488,
    Ia32VmxCr4Fixed1 = 0x00000489,
    Ia32VmxVmcsEnum = 0x0000048a,
    Ia32VmxProcBasedCtls2 = 0x0000048b,
    Ia32VmxEptVpidCap = 0x0000048c,
    Ia32VmxTruePinBasedCtls = 0x0000048d,
    Ia32VmxTrueProcBasedCtls = 0x0000048e,
    Ia32VmxTrueExitCtls = 0x0000048f,
    Ia32VmxTrueEntryCtls = 0x00000490,
    Ia32VmxVmFunc = 0x00000491,
    Ia32DebugCtlMSR = 0x000001d9,
    Ia32SysenterCS = 0x00000174,
    Ia32SysenterESP = 0x00000175,
    Ia32SysenterEIP = 0x00000176,
    Ia32SMBase = 0x0000009e,
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
            "VM entry with executive-VMCS pointer not VMXON pointer \
                (when attempting to deactivate the dual-monitor treatment of"
        }
        19 => {
            "VMCALL with non-clear VMCS (when attempting to activate the \
                dual-monitor treatment of SMIs and SMM)"
        }
        20 => "VMCALL with invalid VM-exit control fields",
        22 => {
            "VMCALL with incorrect MSEG revision identifier (when \
            attempting to activate the dual-monitor treatment of SMIs
            and SMM)"
        }
        23 => "VMXOFF under dual-monitor treatment of SMIs and SMM",
        24 => {
            "VMCALL with invalid SMM-monitor features (when attempting to \
                activate the dual-monitor treatment of SMIs and SMM)"
        }
        25 => {
            "VM entry with invalid VM-execution control fields in \
            executive VMCS (when attempting to return from SMM)"
        }
        26 => "VM entry with events blocked by MOV SS.",
        28 => "Invalid operand to INVEPT/INVVPID.",
        _ => "Unknown VM instruction error number.",
    }
}

const FLAGS_CARRY_BIT: u64 = 1;

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
    VirtualProcessorID = 0x00000000,
    PostedIntrNV = 0x00000002,
    GuestESSelector = 0x00000800,
    GuestCSSelector = 0x00000802,
    GuestSSSelector = 0x00000804,
    GuestDSSelector = 0x00000806,
    GuestFSSelector = 0x00000808,
    GuestGSSelector = 0x0000080a,
    GuestLDTRSelector = 0x0000080c,
    GuestTrSelector = 0x0000080e,
    GuestIntrStatus = 0x00000810,
    GuestPmlIndex = 0x00000812,
    HostESSelector = 0x00000c00,
    HostCSSelector = 0x00000c02,
    HostSSSelector = 0x00000c04,
    HostDSSelector = 0x00000c06,
    HostFSSelector = 0x00000c08,
    HostGSSelector = 0x00000c0a,
    HostTrSelector = 0x00000c0c,
    IOBitmapA = 0x00002000,
    IOBitmapAHigh = 0x00002001,
    IOBitmapB = 0x00002002,
    IOBitmapBHigh = 0x00002003,
    MSRBitmap = 0x00002004,
    MSRBitmapHigh = 0x00002005,
    VMExitMSRStoreAddr = 0x00002006,
    VMExitMSRStoreAddrHigh = 0x00002007,
    VMExitMSRLoadAddr = 0x00002008,
    VMExitMSRLoadAddrHigh = 0x00002009,
    VMEntryMSRLoadAddr = 0x0000200a,
    VMEntryMSRLoadAddrHigh = 0x0000200b,
    PMLAddress = 0x0000200e,
    PMLAddressHigh = 0x0000200f,
    TSCOffset = 0x00002010,
    TSCOffsetHigh = 0x00002011,
    VirtualApicPageAddr = 0x00002012,
    VirtualApicPageAddrHigh = 0x00002013,
    APICAccessAddr = 0x00002014,
    APICAccessAddrHigh = 0x00002015,
    PostedIntrDescAddr = 0x00002016,
    PostedIntrDescAddrHigh = 0x00002017,
    EPTPointer = 0x0000201a,
    EPTPointerHigh = 0x0000201b,
    EOIExitBitmap0 = 0x0000201c,
    EOIExitBitmap0High = 0x0000201d,
    EOIExitBitmap1 = 0x0000201e,
    EOIExitBitmap1High = 0x0000201f,
    EOIExitBitmap2 = 0x00002020,
    EOIExitBitmap2High = 0x00002021,
    EOIExitBitmap3 = 0x00002022,
    EOIExitBitmap3High = 0x00002023,
    VMReadBitmap = 0x00002026,
    VMWriteBitmap = 0x00002028,
    XSSExitBitmap = 0x0000202c,
    XSSExitBitmapHigh = 0x0000202d,
    TSXMultiplier = 0x00002032,
    TSXMultiplierHigh = 0x00002033,
    GuestPhysicalAddress = 0x00002400,
    GuestPhysicalAddressHigh = 0x00002401,
    VMCSLinkPointer = 0x00002800,
    VMCSLinkPointerHigh = 0x00002801,
    GuestIA32Debugctl = 0x00002802,
    GuestIA32DebugctlHigh = 0x00002803,
    GuestIA32Pat = 0x00002804,
    GuestIA32PatHigh = 0x00002805,
    GuestIA32Efer = 0x00002806,
    GuestIA32EferHigh = 0x00002807,
    GuestIA32PerfGlobalCtrl = 0x00002808,
    GuestIA32PerfGlobalCtrlHigh = 0x00002809,
    GuestPDPtr0 = 0x0000280a,
    GuestPDPtr0High = 0x0000280b,
    GuestPDPtr1 = 0x0000280c,
    GuestPDPtr1High = 0x0000280d,
    GuestPDPtr2 = 0x0000280e,
    GuestPDPtr2High = 0x0000280f,
    GuestPDPtr3 = 0x00002810,
    GuestPDPtr3High = 0x00002811,
    GuestBndcfgs = 0x00002812,
    GuestBndcfgsHigh = 0x00002813,
    HostIA32Pat = 0x00002c00,
    HostIA32PatHigh = 0x00002c01,
    HostIA32Efer = 0x00002c02,
    HostIA32EferHigh = 0x00002c03,
    HostIA32PerfGlobalCtrl = 0x00002c04,
    HostIA32PerfGlobalCtrlHigh = 0x00002c05,
    PinBasedVMExecControl = 0x00004000,
    CPUBasedVMExecControl = 0x00004002,
    ExceptionBitmap = 0x00004004,
    PageFaultErrorCodeMask = 0x00004006,
    PageFaultErrorCodeMatch = 0x00004008,
    CR3TargetCount = 0x0000400a,
    VMExitControls = 0x0000400c,
    VMExitMsrStoreCount = 0x0000400e,
    VMExitMsrLoadCount = 0x00004010,
    VMEntryControls = 0x00004012,
    VMEntryMsrLoadCount = 0x00004014,
    VMEntryIntrInfoField = 0x00004016,
    VMEntryExceptionErrorCode = 0x00004018,
    VMEntryInstructionLen = 0x0000401a,
    TPRThreshold = 0x0000401c,
    SecondaryVMExecControl = 0x0000401e,
    PLEGap = 0x00004020,
    PLEWindow = 0x00004022,
    VMInstructionError = 0x00004400,
    VMExitReason = 0x00004402,
    VMExitIntrInfo = 0x00004404,
    VMExitIntrErrorCode = 0x00004406,
    IdtVectoringInfoField = 0x00004408,
    IdtVectoringErrorCode = 0x0000440a,
    VMExitInstructionLen = 0x0000440c,
    VMXInstructionInfo = 0x0000440e,
    GuestESLimit = 0x00004800,
    GuestCSLimit = 0x00004802,
    GuestSSLimit = 0x00004804,
    GuestDSLimit = 0x00004806,
    GuestFSLimit = 0x00004808,
    GuestGSLimit = 0x0000480a,
    GuestLDTRLimit = 0x0000480c,
    GuestTrLimit = 0x0000480e,
    GuestGDTRLimit = 0x00004810,
    GuestIDTRLimit = 0x00004812,
    GuestESArBytes = 0x00004814,
    GuestCSArBytes = 0x00004816,
    GuestSSArBytes = 0x00004818,
    GuestDSArBytes = 0x0000481a,
    GuestFSArBytes = 0x0000481c,
    GuestGSArBytes = 0x0000481e,
    GuestLDTRArBytes = 0x00004820,
    GuestTRArBytes = 0x00004822,
    GuestInterruptibilityInfo = 0x00004824,
    GuestActivityState = 0x00004826,
    GuestSysenterCS = 0x0000482a,
    VMXPreemptionTimerValue = 0x0000482e,
    HostIA32SysenterCS = 0x00004c00,
    CR0GuestHostMask = 0x00006000,
    CR4GuestHostMask = 0x00006002,
    CR0ReadShadow = 0x00006004,
    CR4ReadShadow = 0x00006006,
    CR3TargetValue0 = 0x00006008,
    CR3TargetValue1 = 0x0000600a,
    CR3TargetValue2 = 0x0000600c,
    CR3TargetValue3 = 0x0000600e,
    ExitQualification = 0x00006400,
    GuestLinearAddress = 0x0000640a,
    GuestCR0 = 0x00006800,
    GuestCR3 = 0x00006802,
    GuestCR4 = 0x00006804,
    GuestESBase = 0x00006806,
    GuestCSBase = 0x00006808,
    GuestSSBase = 0x0000680a,
    GuestDSBase = 0x0000680c,
    GuestFSBase = 0x0000680e,
    GuestGSBase = 0x00006810,
    GuestLDTRBase = 0x00006812,
    GuestTRBase = 0x00006814,
    GuestGDTRBase = 0x00006816,
    GuestIDTRBase = 0x00006818,
    GuestDR7 = 0x0000681a,
    GuestRSP = 0x0000681c,
    GuestRIP = 0x0000681e,
    GuestRFlags = 0x00006820,
    GuestPendingDbgExceptions = 0x00006822,
    GuestSysenterESP = 0x00006824,
    GuestSysenterEIP = 0x00006826,
    HostCR0 = 0x00006c00,
    HostCR3 = 0x00006c02,
    HostCR4 = 0x00006c04,
    HostFSBase = 0x00006c06,
    HostGSBase = 0x00006c08,
    HostTRBase = 0x00006c0a,
    HostGDTRBase = 0x00006c0c,
    HostIDTRBase = 0x00006c0e,
    HostIA32SysenterESP = 0x00006c10,
    HostIA32SysenterEIP = 0x00006c12,
    HostRSP = 0x00006c14,
    HostRIP = 0x00006c16,
}


pub const fn is_page_aligned(n: u64) -> bool {
    (n & 0xfff) == 0
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
    if ret == 0 { Ok(()) } else { Err(ret) }
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
    if ret == 0 { Ok(val) } else { Err(ret) }
}

pub fn vmwrite(field: VMCSField, val: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
            "xor %eax, %eax; \
             vmwriteq $2, $1; \
             setc %ah; \
             setz %al;"
             : "={eax}"(ret)
             : "r"(field) "r"(val)
             :
            );
    }
    if ret == 0 { Ok(()) } else { Err(ret) }
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
    ((edx as u64) << 32) | (eax as u64)
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
    if ret == 0 { Ok(()) } else { Err(ret) }
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
    if ret == 0 { Ok(()) } else { Err(ret) }
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
    if ret == 0 { Ok(vmcs_phys) } else { Err(ret) }
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
    if ret == 0 { Ok(()) } else { Err(ret) }
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
    if ret == 0 { Ok(()) } else { Err(ret) }
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

pub fn read_tr() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
            "str $0"
            : "=r"(ret)
            :
            :
            );
    }
    ret as u16
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


fn prepare_vmx_memory_region(vmx_region: *mut u8, vmx_region_size: usize) {
    assert!(!vmx_region.is_null());
    assert!(vmx_region_size <= 0x1000);
    assert!(vmx_region_size > mem::size_of::<u32>());

    let vmcs_revision_identifier = get_vmcs_revision_identifier();

    let vmx_region_dwords = vmx_region as *mut u32;

    unsafe {
        *vmx_region_dwords = vmcs_revision_identifier;
    }

    for i in mem::size_of::<u32>()..vmx_region_size {
        unsafe {
            *vmx_region.offset(i as isize) = 0;
        }
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

    if set_lock_bit() != Ok(()) {
        error!("Lock bit not set");
        return Err(());
    }

    set_cr0_bits();
    set_cr4_bits();

    prepare_vmx_memory_region(vmxon_region, vmxon_region_size);

    let result = vmxon(vmxon_region_phys);
    // FIXME: Fix error types
    if result == Ok(()) {
        info!("vmxon succeeded");
        return Ok(());
    } else {
        error!("vmxon failed");
        return Err(());
    }
}

fn vmcs_initialize_host_state() -> Result<(), u32> {
    Ok(())
}

fn vmcs_initialize_guest_segment_fields(
    gdt: *const segmentation::GDTEntry,
    segment: u16,
    access_field: VMCSField,
    limit_field: VMCSField,
    base_field: VMCSField,
    segment_field: VMCSField,
) -> Result<(), u32> {
    let long_mode_bit: u8 = 1 << 5;
    let access: u64;
    let limit: u64;
    let mut base: u64;
    let index = (segment >> 3) as isize;
    unsafe {
        info!(
            "GDT Entry {:x} {} \n\t{:?}",
            segment,
            index,
            *gdt.offset(index)
        );

        access = (((*gdt.offset(index)).access as u64) << 8) |
            (((*gdt.offset(index)).granularity & 0xf0) as u64);

        limit = ((((*gdt.offset(index)).granularity & 0x0f) as u64) << 32) |
            ((*gdt.offset(index)).limit_low as u64);

        base = (((*gdt.offset(index)).base_high as u64) << 24) |
            (((*gdt.offset(index)).base_middle as u64) << 16) |
            ((*gdt.offset(index)).base_low as u64);

        if ((*gdt.offset(index)).granularity & long_mode_bit) != 0 {
            info!("\t64 bit segment");
            base |= ((*gdt.offset(index)).base_highest as u64) << 32;
        }
    }
    vmwrite(access_field, access)?;
    vmwrite(limit_field, limit)?;
    vmwrite(base_field, base)?;
    vmwrite(segment_field, segment as u64)
}

fn vmcs_initialize_guest_state(rsp: u64, rip: u64) -> Result<(), u32> {
    let mut idtr: interrupts::IDTDescriptor = Default::default();
    interrupts::sidt(&mut idtr);
    let mut gdtr: segmentation::GDTDescriptor = Default::default();
    segmentation::sgdt(&mut gdtr);
    let mut ldtr: segmentation::GDTDescriptor = Default::default();
    segmentation::sldt(&mut ldtr);
    let gdt: *const segmentation::GDTEntry = gdtr.base as *const segmentation::GDTEntry;


    vmwrite(VMCSField::GuestCR0, read_cr0())?;
    vmwrite(VMCSField::GuestCR3, read_cr3())?;
    vmwrite(VMCSField::GuestCR4, read_cr4())?;
    vmwrite(VMCSField::GuestDR7, read_db7())?;


    vmwrite(VMCSField::GuestRSP, rsp)?;
    vmwrite(VMCSField::GuestRIP, rip)?;
    vmwrite(VMCSField::GuestRFlags, read_flags() | FLAGS_CARRY_BIT)?;


    vmcs_initialize_guest_segment_fields(
        gdt,
        read_ss(),
        VMCSField::GuestSSArBytes,
        VMCSField::GuestSSLimit,
        VMCSField::GuestSSBase,
        VMCSField::GuestSSSelector,
    )?;
    vmcs_initialize_guest_segment_fields(
        gdt,
        read_cs(),
        VMCSField::GuestCSArBytes,
        VMCSField::GuestCSLimit,
        VMCSField::GuestCSBase,
        VMCSField::GuestCSSelector,
    )?;
    vmcs_initialize_guest_segment_fields(
        gdt,
        read_ds(),
        VMCSField::GuestDSArBytes,
        VMCSField::GuestDSLimit,
        VMCSField::GuestDSBase,
        VMCSField::GuestDSSelector,
    )?;
    vmcs_initialize_guest_segment_fields(
        gdt,
        read_es(),
        VMCSField::GuestESArBytes,
        VMCSField::GuestESLimit,
        VMCSField::GuestESBase,
        VMCSField::GuestESSelector,
    )?;
    vmcs_initialize_guest_segment_fields(
        gdt,
        read_fs(),
        VMCSField::GuestFSArBytes,
        VMCSField::GuestFSLimit,
        VMCSField::GuestFSBase,
        VMCSField::GuestFSSelector,
    )?;
    vmcs_initialize_guest_segment_fields(
        gdt,
        read_gs(),
        VMCSField::GuestGSArBytes,
        VMCSField::GuestGSLimit,
        VMCSField::GuestGSBase,
        VMCSField::GuestGSSelector,
    )?;
    vmcs_initialize_guest_segment_fields(
        gdt,
        read_tr(),
        VMCSField::GuestTRArBytes,
        VMCSField::GuestTrLimit,
        VMCSField::GuestTRBase,
        VMCSField::GuestTrSelector,
    )?;


    vmwrite(VMCSField::GuestIDTRLimit, idtr.limit as u64)?;
    vmwrite(VMCSField::GuestIDTRBase, idtr.base)?;

    vmwrite(VMCSField::GuestGDTRLimit, gdtr.limit as u64)?;
    vmwrite(VMCSField::GuestGDTRBase, gdtr.base)?;

    vmwrite(VMCSField::GuestLDTRLimit, ldtr.limit as u64)?;
    vmwrite(VMCSField::GuestLDTRBase, ldtr.base)?;


    vmwrite(VMCSField::VMCSLinkPointer, 0xffffffff_ffffffff)?;

    vmwrite(VMCSField::GuestIA32Debugctl, rdmsrl(MSR::Ia32DebugCtlMSR))?;

    vmwrite(VMCSField::GuestSysenterCS, rdmsrl(MSR::Ia32SysenterCS))?;
    vmwrite(VMCSField::GuestSysenterESP, rdmsrl(MSR::Ia32SysenterESP))?;
    vmwrite(VMCSField::GuestSysenterEIP, rdmsrl(MSR::Ia32SysenterEIP))?;

    Ok(())
}

fn vmcs_initialize_vm_control_values() {
    // Simon, this is your place to ☆shine☆!
}


pub fn disable() {
    vmxoff();
    info!("vmxoff");
}

// This must be a macro because otherwise the value will be perturbed by the
// function prologue.
macro_rules! read_rsp {
    () => (
        unsafe {
            let rsp: u64;
            asm!("mov %rsp, $0;" : "=r"(rsp));
            rsp
        }
    )
}

fn is_in_vm() -> (bool, u64) {
    let rip: u64;
    let rflags: u64;
    unsafe {
        asm!(
            "clc             # Clear carry bit.
                             # Before entering the VM, we set the carry bit.
                             # We will enter the VM at the next instruction.
                             # If the carry bit is set after the next instruction,
                             # we must be in a VM.
                             # This hack is borrowed from SimpleVisor.
             lea 0(%rip), $0 # Save the rip so we can start the vm right here.
             pushf           # Push the flags to the stack so we can inspect them.
             pop $1          # Get the flags.
        "
        : "=r"(rip), "=r"(rflags)
        );
    }
    ((rflags & FLAGS_CARRY_BIT) != 0, rip)
}

pub fn load_vm(vmcs: *mut u8, vmcs_phys: u64, vmcs_size: usize) -> Result<(), ()> {

    let rsp = read_rsp!();
    let (in_vm, rip) = is_in_vm();
    if in_vm {
        return Ok(());
    }

    assert!(is_page_aligned(vmcs as u64));
    assert!(is_page_aligned(vmcs_phys));

    prepare_vmx_memory_region(vmcs, vmcs_size);

    if vmclear(vmcs_phys) != Ok(()) {
        return Err(());
    }

    if vmptrld(vmcs_phys) != Ok(()) {
        return Err(());
    }

    if vmcs_initialize_host_state() != Ok(()) {
        return Err(());
    }

    if vmcs_initialize_guest_state(rsp, rip) != Ok(()) {
        return Err(());
    }
    vmcs_initialize_vm_control_values();

    if vmlaunch() != Ok(()) {
        match vmread(VMCSField::VMInstructionError) {
            Ok(vm_instruction_error_number) => {
                error!(
                    "Failed to launch VM because {} ({})",
                    vm_instruction_error_number_message(vm_instruction_error_number),
                    vm_instruction_error_number
                )
            }
            Err(e) => error!("VMLaunch failed with {}", e),
        }
        return Err(());
    }

    Ok(())
}

pub fn unload_vm() {
    match vmptrst() {
        Ok(vmcs_phys) => {
            match vmclear(vmcs_phys) {
                Ok(()) => {}
                Err(code) => error!("vmclear failed with error code {}", code),
            }
        }
        Err(_) => {}
    }
}
