#![allow(non_upper_case_globals)]
#![allow(unused)]

#[repr(u64)]
pub enum VmcsField {
    VirtualProcessorID = 0x0000_0000,
    PostedIntrNV = 0x0000_0002,
    GuestEsSelector = 0x0000_0800,
    GuestCsSelector = 0x0000_0802,
    GuestSsSelector = 0x0000_0804,
    GuestDsSelector = 0x0000_0806,
    GuestFSelector = 0x0000_0808,
    GuestGsSelector = 0x0000_080a,
    GuestLdtrSelector = 0x0000_080c,
    GuestTrSelector = 0x0000_080e,
    GuestIntrStatus = 0x0000_0810,
    GuestPmlIndex = 0x0000_0812,
    HostEsSelector = 0x0000_0c00,
    HostCsSelector = 0x0000_0c02,
    HostSsSelector = 0x0000_0c04,
    HostDsSelector = 0x0000_0c06,
    HostFsSelector = 0x0000_0c08,
    HostGsSelector = 0x0000_0c0a,
    HostTrSelector = 0x0000_0c0c,
    IoBitmapA = 0x0000_2000,
    IoBitmapAHigh = 0x0000_2001,
    IoBitmapB = 0x0000_2002,
    IoBitmapBHigh = 0x0000_2003,
    MsrBitmap = 0x0000_2004,
    MsrBitmapHigh = 0x0000_2005,
    VmExitMsrStoreAddr = 0x0000_2006,
    VmExitMsrStoreAddrHigh = 0x0000_2007,
    VmExitMsrLoadAddr = 0x0000_2008,
    VmExitMsrLoadAddrHigh = 0x0000_2009,
    VmEntryMsrLoadAddr = 0x0000_200a,
    VmEntryMsrLoadAddrHigh = 0x0000_200b,
    PMLAddress = 0x0000_200e,
    PMLAddressHigh = 0x0000_200f,
    TscOffset = 0x0000_2010,
    TscOffsetHigh = 0x0000_2011,
    VirtualApicPageAddr = 0x0000_2012,
    VirtualApicPageAddrHigh = 0x0000_2013,
    APICAccessAddr = 0x0000_2014,
    APICAccessAddrHigh = 0x0000_2015,
    PostedIntrDescAddr = 0x0000_2016,
    PostedIntrDescAddrHigh = 0x0000_2017,
    EPTPointer = 0x0000_201a,
    EPTPointerHigh = 0x0000_201b,
    EoiExitBitmap0 = 0x0000_201c,
    EoiExitBitmap0High = 0x0000_201d,
    EoiExitBitmap1 = 0x0000_201e,
    EoiExitBitmap1High = 0x0000_201f,
    EoiExitBitmap2 = 0x0000_2020,
    EoiExitBitmap2High = 0x0000_2021,
    EoiExitBitmap3 = 0x0000_2022,
    EoiExitBitmap3High = 0x0000_2023,
    VmReadBitmap = 0x0000_2026,
    VmWriteBitmap = 0x0000_2028,
    XssExitBitmap = 0x0000_202c,
    XssExitBitmapHigh = 0x0000_202d,
    TsxMultiplier = 0x0000_2032,
    TsxMultiplierHigh = 0x0000_2033,
    GuestPhysicalAddress = 0x0000_2400,
    GuestPhysicalAddressHigh = 0x0000_2401,
    VmcsLinkPointer = 0x0000_2800,
    VmcsLinkPointerHigh = 0x0000_2801,
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
    PinBasedVmExecControl = 0x0000_4000,
    CpuBasedVmExecControl = 0x0000_4002,
    ExceptIonBitmap = 0x0000_4004,
    PageFaultErrorCodeMask = 0x0000_4006,
    PageFaultErrorCodeMatch = 0x0000_4008,
    Cr3TargetCount = 0x0000_400a,
    VmExitControls = 0x0000_400c,
    VmExitMsrStoreCount = 0x0000_400e,
    VmExitMsrLoadCount = 0x0000_4010,
    VmEntryControls = 0x0000_4012,
    VmEntryMsrLoadCount = 0x0000_4014,
    VmEntryIntrInfoField = 0x0000_4016,
    VmEntryExceptIonErrorCode = 0x0000_4018,
    VmEntryInstructionLen = 0x0000_401a,
    TPRThreshold = 0x0000_401c,
    SecondaryVmExecControl = 0x0000_401e,
    PLEGap = 0x0000_4020,
    PLEWindow = 0x0000_4022,
    VmInstructionError = 0x0000_4400,
    VmExitReason = 0x0000_4402,
    VmExitIntrInfo = 0x0000_4404,
    VmExitIntrErrorCode = 0x0000_4406,
    IdtVectoringInfoField = 0x0000_4408,
    IdtVectoringErrorCode = 0x0000_440a,
    VmExitInstructionLen = 0x0000_440c,
    VmxInstructionInfo = 0x0000_440e,
    GuestEsLimit = 0x0000_4800,
    GuestCsLimit = 0x0000_4802,
    GuestSsLimit = 0x0000_4804,
    GuestDsLimit = 0x0000_4806,
    GuestFsLimit = 0x0000_4808,
    GuestGsLimit = 0x0000_480a,
    GuestLdtrLimit = 0x0000_480c,
    GuestTrLimit = 0x0000_480e,
    GuestGdtrLimit = 0x0000_4810,
    GuestIdtrLimit = 0x0000_4812,
    GuestEsArBytes = 0x0000_4814,
    GuestCsArBytes = 0x0000_4816,
    GuestSsArBytes = 0x0000_4818,
    GuestDsArBytes = 0x0000_481a,
    GuestFsArBytes = 0x0000_481c,
    GuestGsArBytes = 0x0000_481e,
    GuestLdtrArBytes = 0x0000_4820,
    GuestTrArBytes = 0x0000_4822,
    GuestInterruptibilityInfo = 0x0000_4824,
    GuestActivityState = 0x0000_4826,
    GuestSysenterCs = 0x0000_482a,
    VmxPreemptionTimerValue = 0x0000_482e,
    HostIA32SysenterCs = 0x0000_4c00,
    Cr0GuestHostMask = 0x0000_6000,
    Cr4GuestHostMask = 0x0000_6002,
    Cr0ReadShadow = 0x0000_6004,
    Cr4ReadShadow = 0x0000_6006,
    Cr3TargetValue0 = 0x0000_6008,
    Cr3TargetValue1 = 0x0000_600a,
    Cr3TargetValue2 = 0x0000_600c,
    Cr3TargetValue3 = 0x0000_600e,
    ExitQualificatIon = 0x0000_6400,
    GuestLinearAddress = 0x0000_640a,
    GuestCr0 = 0x0000_6800,
    GuestCr3 = 0x0000_6802,
    GuestCr4 = 0x0000_6804,
    GuestEsBase = 0x0000_6806,
    GuestCsBase = 0x0000_6808,
    GuestSsBase = 0x0000_680a,
    GuestDsBase = 0x0000_680c,
    GuestFsBase = 0x0000_680e,
    GuestGsBase = 0x0000_6810,
    GuestLdtrBase = 0x0000_6812,
    GuestTrBase = 0x0000_6814,
    GuestGdtrBase = 0x0000_6816,
    GuestIdtrBase = 0x0000_6818,
    GuestDr7 = 0x0000_681a,
    GuestRsp = 0x0000_681c,
    GuestRip = 0x0000_681e,
    GuestRFlags = 0x0000_6820,
    GuestPendingDbgExceptions = 0x0000_6822,
    GuestSysenterEsp = 0x0000_6824,
    GuestSysenterEip = 0x0000_6826,
    HostCr0 = 0x0000_6c00,
    HostCr3 = 0x0000_6c02,
    HostCr4 = 0x0000_6c04,
    HostFsBase = 0x0000_6c06,
    HostGsBase = 0x0000_6c08,
    HostTrBase = 0x0000_6c0a,
    HostGdtrBase = 0x0000_6c0c,
    HostIdtrBase = 0x0000_6c0e,
    HostIA32SysenterEsp = 0x0000_6c10,
    HostIA32SysenterEip = 0x0000_6c12,
    HostRsp = 0x0000_6c14,
    HostRip = 0x0000_6c16,
}

pub const PinBasedControlsExternalInterruptExiting: u64 = 1 << 0;
pub const PinBasedControlsNmiExiting: u64 = 1 << 3;
pub const PinBasedControlsVirtualNmi: u64 = 1 << 5;
pub const PinBasedControlsVmxPreemption: u64 = 1 << 6;
pub const PinBasedControlsPostedInterrupts: u64 = 1 << 7;

pub const CpuBasedControlsInterruptWindowExiting: u64 = 1 << 2;
pub const CpuBasedControlsTscOffsetting: u64 = 1 << 3;
pub const CpuBasedControlsHltExiting: u64 = 1 << 7;
pub const CpuBasedControlsInvlpgExiting: u64 = 1 << 9;
pub const CpuBasedControlsMwaitExiting: u64 = 1 << 10;
pub const CpuBasedControlsRdpmcExiting: u64 = 1 << 11;
pub const CpuBasedControlsRdtscExiting: u64 = 1 << 12;
pub const CpuBasedControlsCr3LdExiting: u64 = 1 << 15;
pub const CpuBasedControlsCr3StExiting: u64 = 1 << 16;
pub const CpuBasedControlsCr8LdExiting: u64 = 1 << 19;
pub const CpuBasedControlsCr8StExiting: u64 = 1 << 20;
pub const CpuBasedControlsTprShadow: u64 = 1 << 21;
pub const CpuBasedControlsNmiWindowExiting: u64 = 1 << 22;
pub const CpuBasedControlsMovDrExiting: u64 = 1 << 23;
pub const CpuBasedControlsIoExiting: u64 = 1 << 24;
pub const CpuBasedControlsIoBitmaps: u64 = 1 << 25;
pub const CpuBasedControlsMonitorTrapFlagEnable: u64 = 1 << 27;
pub const CpuBasedControlsMsrBitmaps: u64 = 1 << 28;
pub const CpuBasedControlsMonitorExiting: u64 = 1 << 29;
pub const CpuBasedControlsPauseExiting: u64 = 1 << 30;
pub const CpuBasedControlsSecondaryEnable: u64 = 1 << 31;

pub const SecondaryCpuBasedControlsVirtualApic: u64 = 1 << 0;
pub const SecondaryCpuBasedControlsEptEnable: u64 = 1 << 1;
pub const SecondaryCpuBasedControlsDtExiting: u64 = 1 << 2;
pub const SecondaryCpuBasedControlsRdtscpEnable: u64 = 1 << 3;
pub const SecondaryCpuBasedControlsX2ApicEnable: u64 = 1 << 4;
pub const SecondaryCpuBasedControlsVpidEnable: u64 = 1 << 5;
pub const SecondaryCpuBasedControlsWbinvdExiting: u64 = 1 << 6;
pub const SecondaryCpuBasedControlsUnrestrictedGuest: u64 = 1 << 7;
pub const SecondaryCpuBasedControlsVirtualApicRegister: u64 = 1 << 8;
pub const SecondaryCpuBasedControlsVirtualInterruptEnable: u64 = 1 << 9;
pub const SecondaryCpuBasedControlsPauseLoopExiting: u64 = 1 << 10;
pub const SecondaryCpuBasedControlsRdrandExiting: u64 = 1 << 11;
pub const SecondaryCpuBasedControlsInvpcidEnable: u64 = 1 << 12;
pub const SecondaryCpuBasedControlsVmfuncEnable: u64 = 1 << 13;
pub const SecondaryCpuBasedControlsVmcsShadow: u64 = 1 << 14;
pub const SecondaryCpuBasedControlsEnclsExiting: u64 = 1 << 15;
pub const SecondaryCpuBasedControlsRdseedExiting: u64 = 1 << 16;
pub const SecondaryCpuBasedControlsPmlEnable: u64 = 1 << 17;
pub const SecondaryCpuBasedControlsEptVeEnable: u64 = 1 << 18;
pub const SecondaryCpuBasedControlsPtConcealVmx: u64 = 1 << 19;
pub const SecondaryCpuBasedControlsXSavesEnable: u64 = 1 << 20;
pub const SecondaryCpuBasedControlsEptExecuteControl: u64 = 1 << 22;
pub const SecondaryCpuBasedControlsTscScalingEnable: u64 = 1 << 25;

// Table 24-11  sectIon 24.7.1 vol3c
pub const VmExitIa32eMode: u64 = 1 << 9;
pub const VmExitAcknowledgeInterruptOnExit: u64 = 1 << 15;
pub const VmExitConcealVmxFromPt: u64 = 1 << 24;

// Table 24-13  sectIon 24.8.1 vol3c
pub const VmEntryIa32eMode: u64 = 1 << 9;
