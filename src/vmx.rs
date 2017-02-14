#[repr(u32)]
pub enum MSR {
    EFER = 0xc0000080,
}

#[repr(u64)]
pub enum VMCSField {
    VirtualProcessorID = 0x00000000,
    PostedIntrNV = 0x00000002,
    GuestESSelector = 0x00000800,
    GuestCSSelector = 0x00000802,
    GuestSSSelector = 0x00000804,
    GuestDSSelector = 0x00000806,
    GuestFSelector = 0x00000808,
    GuestGSSelector = 0x0000080a,
    GuestLDTRSelector = 0x0000080c,
    GuestTrSelector = 0x0000080e,
    GuestIntrStatus = 0x00000810,
    GuestPmlIndex = 0x00000812,
    HostESSelector = 0x00000c00,
    HostCSSelector = 0x00000c02,
    HostSSSelector = 0x00000c04,
    HostDSSelector = 0x00000c06,
    HostFSelector = 0x00000c08,
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
    VMcsLinkPointer = 0x00002800,
    VMcsLinkPointerHigh = 0x00002801,
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
    GuestFsLimit = 0x00004808,
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
    GuestFsBase = 0x0000680e,
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
             vmread $2, $1; \
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
