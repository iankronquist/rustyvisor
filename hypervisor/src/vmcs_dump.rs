use crate::vmcs;
use crate::vmcs_fields::VmcsField;
use crate::vmx::vmread;
use log::debug;

pub fn dump() {
    let val = vmread(VmcsField::VirtualProcessorID)
        .unwrap_or(0xbadc0de);
    debug!("VirtualProcessorID: {:x}", val);
    let val = vmread(VmcsField::PostedIntrNV)
        .unwrap_or(0xbadc0de);
    debug!("PostedIntrNV: {:x}", val);
    let val = vmread(VmcsField::GuestEsSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestEsSelector: {:x}", val);
    let val = vmread(VmcsField::GuestCsSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestCsSelector: {:x}", val);
    let val = vmread(VmcsField::GuestSsSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestSsSelector: {:x}", val);
    let val = vmread(VmcsField::GuestDsSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestDsSelector: {:x}", val);
    let val = vmread(VmcsField::GuestFsSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestFsSelector: {:x}", val);
    let val = vmread(VmcsField::GuestGsSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestGsSelector: {:x}", val);
    let val = vmread(VmcsField::GuestLdtrSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestLdtrSelector: {:x}", val);
    let val = vmread(VmcsField::GuestTrSelector)
        .unwrap_or(0xbadc0de);
    debug!("GuestTrSelector: {:x}", val);
    let val = vmread(VmcsField::GuestIntrStatus)
        .unwrap_or(0xbadc0de);
    debug!("GuestIntrStatus: {:x}", val);
    let val = vmread(VmcsField::GuestPmlIndex)
        .unwrap_or(0xbadc0de);
    debug!("GuestPmlIndex: {:x}", val);
    let val = vmread(VmcsField::HostEsSelector)
        .unwrap_or(0xbadc0de);
    debug!("HostEsSelector: {:x}", val);
    let val = vmread(VmcsField::HostCsSelector)
        .unwrap_or(0xbadc0de);
    debug!("HostCsSelector: {:x}", val);
    let val = vmread(VmcsField::HostSsSelector)
        .unwrap_or(0xbadc0de);
    debug!("HostSsSelector: {:x}", val);
    let val = vmread(VmcsField::HostDsSelector)
        .unwrap_or(0xbadc0de);
    debug!("HostDsSelector: {:x}", val);
    let val = vmread(VmcsField::HostFsSelector)
        .unwrap_or(0xbadc0de);
    debug!("HostFsSelector: {:x}", val);
    let val = vmread(VmcsField::HostGsSelector)
        .unwrap_or(0xbadc0de);
    debug!("HostGsSelector: {:x}", val);
    let val = vmread(VmcsField::HostTrSelector)
        .unwrap_or(0xbadc0de);
    debug!("HostTrSelector: {:x}", val);
    let val = vmread(VmcsField::IoBitmapA)
        .unwrap_or(0xbadc0de);
    debug!("IoBitmapA: {:x}", val);
    let val = vmread(VmcsField::IoBitmapAHigh)
        .unwrap_or(0xbadc0de);
    debug!("IoBitmapAHigh: {:x}", val);
    let val = vmread(VmcsField::IoBitmapB)
        .unwrap_or(0xbadc0de);
    debug!("IoBitmapB: {:x}", val);
    let val = vmread(VmcsField::IoBitmapBHigh)
        .unwrap_or(0xbadc0de);
    debug!("IoBitmapBHigh: {:x}", val);
    let val = vmread(VmcsField::MsrBitmap)
        .unwrap_or(0xbadc0de);
    debug!("MsrBitmap: {:x}", val);
    let val = vmread(VmcsField::MsrBitmapHigh)
        .unwrap_or(0xbadc0de);
    debug!("MsrBitmapHigh: {:x}", val);
    let val = vmread(VmcsField::VmExitMsrStoreAddr)
        .unwrap_or(0xbadc0de);
    debug!("VmExitMsrStoreAddr: {:x}", val);
    let val = vmread(VmcsField::VmExitMsrStoreAddrHigh)
        .unwrap_or(0xbadc0de);
    debug!("VmExitMsrStoreAddrHigh: {:x}", val);
    let val = vmread(VmcsField::VmExitMsrLoadAddr)
        .unwrap_or(0xbadc0de);
    debug!("VmExitMsrLoadAddr: {:x}", val);
    let val = vmread(VmcsField::VmExitMsrLoadAddrHigh)
        .unwrap_or(0xbadc0de);
    debug!("VmExitMsrLoadAddrHigh: {:x}", val);
    let val = vmread(VmcsField::VmEntryMsrLoadAddr)
        .unwrap_or(0xbadc0de);
    debug!("VmEntryMsrLoadAddr: {:x}", val);
    let val = vmread(VmcsField::VmEntryMsrLoadAddrHigh)
        .unwrap_or(0xbadc0de);
    debug!("VmEntryMsrLoadAddrHigh: {:x}", val);
    let val = vmread(VmcsField::PMLAddress)
        .unwrap_or(0xbadc0de);
    debug!("PMLAddress: {:x}", val);
    let val = vmread(VmcsField::PMLAddressHigh)
        .unwrap_or(0xbadc0de);
    debug!("PMLAddressHigh: {:x}", val);
    let val = vmread(VmcsField::TscOffset)
        .unwrap_or(0xbadc0de);
    debug!("TscOffset: {:x}", val);
    let val = vmread(VmcsField::TscOffsetHigh)
        .unwrap_or(0xbadc0de);
    debug!("TscOffsetHigh: {:x}", val);
    let val = vmread(VmcsField::VirtualApicPageAddr)
        .unwrap_or(0xbadc0de);
    debug!("VirtualApicPageAddr: {:x}", val);
    let val = vmread(VmcsField::VirtualApicPageAddrHigh)
        .unwrap_or(0xbadc0de);
    debug!("VirtualApicPageAddrHigh: {:x}", val);
    let val = vmread(VmcsField::APICAccessAddr)
        .unwrap_or(0xbadc0de);
    debug!("APICAccessAddr: {:x}", val);
    let val = vmread(VmcsField::APICAccessAddrHigh)
        .unwrap_or(0xbadc0de);
    debug!("APICAccessAddrHigh: {:x}", val);
    let val = vmread(VmcsField::PostedIntrDescAddr)
        .unwrap_or(0xbadc0de);
    debug!("PostedIntrDescAddr: {:x}", val);
    let val = vmread(VmcsField::PostedIntrDescAddrHigh)
        .unwrap_or(0xbadc0de);
    debug!("PostedIntrDescAddrHigh: {:x}", val);
    let val = vmread(VmcsField::EPTPointer)
        .unwrap_or(0xbadc0de);
    debug!("EPTPointer: {:x}", val);
    let val = vmread(VmcsField::EPTPointerHigh)
        .unwrap_or(0xbadc0de);
    debug!("EPTPointerHigh: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap0)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap0: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap0High)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap0High: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap1)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap1: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap1High)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap1High: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap2)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap2: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap2High)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap2High: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap3)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap3: {:x}", val);
    let val = vmread(VmcsField::EoiExitBitmap3High)
        .unwrap_or(0xbadc0de);
    debug!("EoiExitBitmap3High: {:x}", val);
    let val = vmread(VmcsField::VmReadBitmap)
        .unwrap_or(0xbadc0de);
    debug!("VmReadBitmap: {:x}", val);
    let val = vmread(VmcsField::VmWriteBitmap)
        .unwrap_or(0xbadc0de);
    debug!("VmWriteBitmap: {:x}", val);
    let val = vmread(VmcsField::XssExitBitmap)
        .unwrap_or(0xbadc0de);
    debug!("XssExitBitmap: {:x}", val);
    let val = vmread(VmcsField::XssExitBitmapHigh)
        .unwrap_or(0xbadc0de);
    debug!("XssExitBitmapHigh: {:x}", val);
    let val = vmread(VmcsField::TsxMultiplier)
        .unwrap_or(0xbadc0de);
    debug!("TsxMultiplier: {:x}", val);
    let val = vmread(VmcsField::TsxMultiplierHigh)
        .unwrap_or(0xbadc0de);
    debug!("TsxMultiplierHigh: {:x}", val);
    let val = vmread(VmcsField::GuestPhysicalAddress)
        .unwrap_or(0xbadc0de);
    debug!("GuestPhysicalAddress: {:x}", val);
    let val = vmread(VmcsField::GuestPhysicalAddressHigh)
        .unwrap_or(0xbadc0de);
    debug!("GuestPhysicalAddressHigh: {:x}", val);
    let val = vmread(VmcsField::VmcsLinkPointer)
        .unwrap_or(0xbadc0de);
    debug!("VmcsLinkPointer: {:x}", val);
    let val = vmread(VmcsField::VmcsLinkPointerHigh)
        .unwrap_or(0xbadc0de);
    debug!("VmcsLinkPointerHigh: {:x}", val);
    let val = vmread(VmcsField::GuestIA32Debugctl)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32Debugctl: {:x}", val);
    let val = vmread(VmcsField::GuestIA32DebugctlHigh)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32DebugctlHigh: {:x}", val);
    let val = vmread(VmcsField::GuestIA32Pat)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32Pat: {:x}", val);
    let val = vmread(VmcsField::GuestIA32PatHigh)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32PatHigh: {:x}", val);
    let val = vmread(VmcsField::GuestIA32Efer)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32Efer: {:x}", val);
    let val = vmread(VmcsField::GuestIA32EferHigh)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32EferHigh: {:x}", val);
    let val = vmread(VmcsField::GuestIA32PerfGlobalCtrl)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32PerfGlobalCtrl: {:x}", val);
    let val = vmread(VmcsField::GuestIA32PerfGlobalCtrlHigh)
        .unwrap_or(0xbadc0de);
    debug!("GuestIA32PerfGlobalCtrlHigh: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr0)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr0: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr0High)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr0High: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr1)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr1: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr1High)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr1High: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr2)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr2: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr2High)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr2High: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr3)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr3: {:x}", val);
    let val = vmread(VmcsField::GuestPDPtr3High)
        .unwrap_or(0xbadc0de);
    debug!("GuestPDPtr3High: {:x}", val);
    let val = vmread(VmcsField::GuestBndcfgs)
        .unwrap_or(0xbadc0de);
    debug!("GuestBndcfgs: {:x}", val);
    let val = vmread(VmcsField::GuestBndcfgsHigh)
        .unwrap_or(0xbadc0de);
    debug!("GuestBndcfgsHigh: {:x}", val);
    let val = vmread(VmcsField::HostIA32Pat)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32Pat: {:x}", val);
    let val = vmread(VmcsField::HostIA32PatHigh)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32PatHigh: {:x}", val);
    let val = vmread(VmcsField::HostIA32Efer)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32Efer: {:x}", val);
    let val = vmread(VmcsField::HostIA32EferHigh)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32EferHigh: {:x}", val);
    let val = vmread(VmcsField::HostIA32PerfGlobalCtrl)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32PerfGlobalCtrl: {:x}", val);
    let val = vmread(VmcsField::HostIA32PerfGlobalCtrlHigh)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32PerfGlobalCtrlHigh: {:x}", val);
    let val = vmread(VmcsField::PinBasedVmExecControl)
        .unwrap_or(0xbadc0de);
    debug!("PinBasedVmExecControl: {:x}", val);
    let val = vmread(VmcsField::CpuBasedVmExecControl)
        .unwrap_or(0xbadc0de);
    debug!("CpuBasedVmExecControl: {:x}", val);
    let val = vmread(VmcsField::ExceptIonBitmap)
        .unwrap_or(0xbadc0de);
    debug!("ExceptIonBitmap: {:x}", val);
    let val = vmread(VmcsField::PageFaultErrorCodeMask)
        .unwrap_or(0xbadc0de);
    debug!("PageFaultErrorCodeMask: {:x}", val);
    let val = vmread(VmcsField::PageFaultErrorCodeMatch)
        .unwrap_or(0xbadc0de);
    debug!("PageFaultErrorCodeMatch: {:x}", val);
    let val = vmread(VmcsField::Cr3TargetCount)
        .unwrap_or(0xbadc0de);
    debug!("Cr3TargetCount: {:x}", val);
    let val = vmread(VmcsField::VmExitControls)
        .unwrap_or(0xbadc0de);
    debug!("VmExitControls: {:x}", val);
    let val = vmread(VmcsField::VmExitMsrStoreCount)
        .unwrap_or(0xbadc0de);
    debug!("VmExitMsrStoreCount: {:x}", val);
    let val = vmread(VmcsField::VmExitMsrLoadCount)
        .unwrap_or(0xbadc0de);
    debug!("VmExitMsrLoadCount: {:x}", val);
    let val = vmread(VmcsField::VmEntryControls)
        .unwrap_or(0xbadc0de);
    debug!("VmEntryControls: {:x}", val);
    let val = vmread(VmcsField::VmEntryMsrLoadCount)
        .unwrap_or(0xbadc0de);
    debug!("VmEntryMsrLoadCount: {:x}", val);
    let val = vmread(VmcsField::VmEntryIntrInfoField)
        .unwrap_or(0xbadc0de);
    debug!("VmEntryIntrInfoField: {:x}", val);
    let val = vmread(VmcsField::VmEntryExceptIonErrorCode)
        .unwrap_or(0xbadc0de);
    debug!("VmEntryExceptIonErrorCode: {:x}", val);
    let val = vmread(VmcsField::VmEntryInstructionLen)
        .unwrap_or(0xbadc0de);
    debug!("VmEntryInstructionLen: {:x}", val);
    let val = vmread(VmcsField::TPRThreshold)
        .unwrap_or(0xbadc0de);
    debug!("TPRThreshold: {:x}", val);
    let val = vmread(VmcsField::SecondaryVmExecControl)
        .unwrap_or(0xbadc0de);
    debug!("SecondaryVmExecControl: {:x}", val);
    let val = vmread(VmcsField::PLEGap)
        .unwrap_or(0xbadc0de);
    debug!("PLEGap: {:x}", val);
    let val = vmread(VmcsField::PLEWindow)
        .unwrap_or(0xbadc0de);
    debug!("PLEWindow: {:x}", val);
    let val = vmread(VmcsField::VmInstructionError)
        .unwrap_or(0xbadc0de);
    debug!("VmInstructionError: {:x}", val);
    debug!("VmInstructionError: {}", vmcs::vm_instruction_error_number_message(val));
    let val = vmread(VmcsField::VmExitReason)
        .unwrap_or(0xbadc0de);
    debug!("VmExitReason: {:x}", val);
    let val = vmread(VmcsField::VmExitIntrInfo)
        .unwrap_or(0xbadc0de);
    debug!("VmExitIntrInfo: {:x}", val);
    let val = vmread(VmcsField::VmExitIntrErrorCode)
        .unwrap_or(0xbadc0de);
    debug!("VmExitIntrErrorCode: {:x}", val);
    let val = vmread(VmcsField::IdtVectoringInfoField)
        .unwrap_or(0xbadc0de);
    debug!("IdtVectoringInfoField: {:x}", val);
    let val = vmread(VmcsField::IdtVectoringErrorCode)
        .unwrap_or(0xbadc0de);
    debug!("IdtVectoringErrorCode: {:x}", val);
    let val = vmread(VmcsField::VmExitInstructionLen)
        .unwrap_or(0xbadc0de);
    debug!("VmExitInstructionLen: {:x}", val);
    let val = vmread(VmcsField::VmxInstructionInfo)
        .unwrap_or(0xbadc0de);
    debug!("VmxInstructionInfo: {:x}", val);
    let val = vmread(VmcsField::GuestEsLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestEsLimit: {:x}", val);
    let val = vmread(VmcsField::GuestCsLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestCsLimit: {:x}", val);
    let val = vmread(VmcsField::GuestSsLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestSsLimit: {:x}", val);
    let val = vmread(VmcsField::GuestDsLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestDsLimit: {:x}", val);
    let val = vmread(VmcsField::GuestFsLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestFsLimit: {:x}", val);
    let val = vmread(VmcsField::GuestGsLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestGsLimit: {:x}", val);
    let val = vmread(VmcsField::GuestLdtrLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestLdtrLimit: {:x}", val);
    let val = vmread(VmcsField::GuestTrLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestTrLimit: {:x}", val);
    let val = vmread(VmcsField::GuestGdtrLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestGdtrLimit: {:x}", val);
    let val = vmread(VmcsField::GuestIdtrLimit)
        .unwrap_or(0xbadc0de);
    debug!("GuestIdtrLimit: {:x}", val);
    let val = vmread(VmcsField::GuestEsArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestEsArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestCsArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestCsArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestSsArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestSsArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestDsArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestDsArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestFsArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestFsArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestGsArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestGsArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestLdtrArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestLdtrArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestTrArBytes)
        .unwrap_or(0xbadc0de);
    debug!("GuestTrArBytes: {:x}", val);
    let val = vmread(VmcsField::GuestInterruptibilityInfo)
        .unwrap_or(0xbadc0de);
    debug!("GuestInterruptibilityInfo: {:x}", val);
    let val = vmread(VmcsField::GuestActivityState)
        .unwrap_or(0xbadc0de);
    debug!("GuestActivityState: {:x}", val);
    let val = vmread(VmcsField::GuestSysenterCs)
        .unwrap_or(0xbadc0de);
    debug!("GuestSysenterCs: {:x}", val);
    let val = vmread(VmcsField::VmxPreemptionTimerValue)
        .unwrap_or(0xbadc0de);
    debug!("VmxPreemptionTimerValue: {:x}", val);
    let val = vmread(VmcsField::HostIA32SysenterCs)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32SysenterCs: {:x}", val);
    let val = vmread(VmcsField::Cr0GuestHostMask)
        .unwrap_or(0xbadc0de);
    debug!("Cr0GuestHostMask: {:x}", val);
    let val = vmread(VmcsField::Cr4GuestHostMask)
        .unwrap_or(0xbadc0de);
    debug!("Cr4GuestHostMask: {:x}", val);
    let val = vmread(VmcsField::Cr0ReadShadow)
        .unwrap_or(0xbadc0de);
    debug!("Cr0ReadShadow: {:x}", val);
    let val = vmread(VmcsField::Cr4ReadShadow)
        .unwrap_or(0xbadc0de);
    debug!("Cr4ReadShadow: {:x}", val);
    let val = vmread(VmcsField::Cr3TargetValue0)
        .unwrap_or(0xbadc0de);
    debug!("Cr3TargetValue0: {:x}", val);
    let val = vmread(VmcsField::Cr3TargetValue1)
        .unwrap_or(0xbadc0de);
    debug!("Cr3TargetValue1: {:x}", val);
    let val = vmread(VmcsField::Cr3TargetValue2)
        .unwrap_or(0xbadc0de);
    debug!("Cr3TargetValue2: {:x}", val);
    let val = vmread(VmcsField::Cr3TargetValue3)
        .unwrap_or(0xbadc0de);
    debug!("Cr3TargetValue3: {:x}", val);
    let val = vmread(VmcsField::ExitQualificatIon)
        .unwrap_or(0xbadc0de);
    debug!("ExitQualificatIon: {:x}", val);
    let val = vmread(VmcsField::GuestLinearAddress)
        .unwrap_or(0xbadc0de);
    debug!("GuestLinearAddress: {:x}", val);
    let val = vmread(VmcsField::GuestCr0)
        .unwrap_or(0xbadc0de);
    debug!("GuestCr0: {:x}", val);
    let val = vmread(VmcsField::GuestCr3)
        .unwrap_or(0xbadc0de);
    debug!("GuestCr3: {:x}", val);
    let val = vmread(VmcsField::GuestCr4)
        .unwrap_or(0xbadc0de);
    debug!("GuestCr4: {:x}", val);
    let val = vmread(VmcsField::GuestEsBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestEsBase: {:x}", val);
    let val = vmread(VmcsField::GuestCsBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestCsBase: {:x}", val);
    let val = vmread(VmcsField::GuestSsBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestSsBase: {:x}", val);
    let val = vmread(VmcsField::GuestDsBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestDsBase: {:x}", val);
    let val = vmread(VmcsField::GuestFsBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestFsBase: {:x}", val);
    let val = vmread(VmcsField::GuestGsBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestGsBase: {:x}", val);
    let val = vmread(VmcsField::GuestLdtrBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestLdtrBase: {:x}", val);
    let val = vmread(VmcsField::GuestTrBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestTrBase: {:x}", val);
    let val = vmread(VmcsField::GuestGdtrBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestGdtrBase: {:x}", val);
    let val = vmread(VmcsField::GuestIdtrBase)
        .unwrap_or(0xbadc0de);
    debug!("GuestIdtrBase: {:x}", val);
    let val = vmread(VmcsField::GuestDr7)
        .unwrap_or(0xbadc0de);
    debug!("GuestDr7: {:x}", val);
    let val = vmread(VmcsField::GuestRsp)
        .unwrap_or(0xbadc0de);
    debug!("GuestRsp: {:x}", val);
    let val = vmread(VmcsField::GuestRip)
        .unwrap_or(0xbadc0de);
    debug!("GuestRip: {:x}", val);
    let val = vmread(VmcsField::GuestRFlags)
        .unwrap_or(0xbadc0de);
    debug!("GuestRFlags: {:x}", val);
    let val = vmread(VmcsField::GuestPendingDbgExceptions)
        .unwrap_or(0xbadc0de);
    debug!("GuestPendingDbgExceptions: {:x}", val);
    let val = vmread(VmcsField::GuestSysenterEsp)
        .unwrap_or(0xbadc0de);
    debug!("GuestSysenterEsp: {:x}", val);
    let val = vmread(VmcsField::GuestSysenterEip)
        .unwrap_or(0xbadc0de);
    debug!("GuestSysenterEip: {:x}", val);
    let val = vmread(VmcsField::HostCr0)
        .unwrap_or(0xbadc0de);
    debug!("HostCr0: {:x}", val);
    let val = vmread(VmcsField::HostCr3)
        .unwrap_or(0xbadc0de);
    debug!("HostCr3: {:x}", val);
    let val = vmread(VmcsField::HostCr4)
        .unwrap_or(0xbadc0de);
    debug!("HostCr4: {:x}", val);
    let val = vmread(VmcsField::HostFsBase)
        .unwrap_or(0xbadc0de);
    debug!("HostFsBase: {:x}", val);
    let val = vmread(VmcsField::HostGsBase)
        .unwrap_or(0xbadc0de);
    debug!("HostGsBase: {:x}", val);
    let val = vmread(VmcsField::HostTrBase)
        .unwrap_or(0xbadc0de);
    debug!("HostTrBase: {:x}", val);
    let val = vmread(VmcsField::HostGdtrBase)
        .unwrap_or(0xbadc0de);
    debug!("HostGdtrBase: {:x}", val);
    let val = vmread(VmcsField::HostIdtrBase)
        .unwrap_or(0xbadc0de);
    debug!("HostIdtrBase: {:x}", val);
    let val = vmread(VmcsField::HostIA32SysenterEsp)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32SysenterEsp: {:x}", val);
    let val = vmread(VmcsField::HostIA32SysenterEip)
        .unwrap_or(0xbadc0de);
    debug!("HostIA32SysenterEip: {:x}", val);
    let val = vmread(VmcsField::HostRsp)
        .unwrap_or(0xbadc0de);
    debug!("HostRsp: {:x}", val);
    let val = vmread(VmcsField::HostRip)
        .unwrap_or(0xbadc0de);
    debug!("HostRip: {:x}", val);
}
