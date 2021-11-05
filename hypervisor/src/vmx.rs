//! Functions for working with the intel VMX extensions.
use ::log::{error, info, trace};
use x86;

use core::{mem, ptr};

use crate::msr::{rdmsr, rdmsrl, wrmsr, Msr};
use crate::vmcs_fields::VmcsField;
use crate::{vmcs, VCpu};

const IA32_FEATURE_CONTROL_LOCK_BIT: u32 = 1 << 0;
const IA32_FEATURE_CONTROL_VMX_ENABLED_OUTSIDE_SMX_BIT: u32 = 1 << 2;

#[repr(u32)]
pub enum CPUIDLeaf {
    ProcessorInfoAndFeatures = 1,
}

#[repr(u32)]
pub enum CPUIDLeafProcessorInfoAndFeaturesECXBits {
    VMXAvailable = 1 << 5,
}

pub const fn is_page_aligned(n: u64) -> bool {
    n.trailing_zeros() >= 12
}

/// Read a field from the currently loaded virtual machine control structure.
///
/// # Safety
/// This must be called from within VMX root operation (i.e. after vmxon happens),
/// and after a vmcs has been loaded with vmptrld.
pub fn vmread(field: VmcsField) -> Result<u64, x86::vmx::VmFail> {
    unsafe { x86::bits64::vmx::vmread(field as u32) }
}

/// Write a field to the currently loaded virtual machine control structure.
///
/// # Safety
/// This must be called from within VMX root operation (i.e. after vmxon happens),
/// and after a vmcs has been loaded with vmptrld.
pub fn vmwrite(field: VmcsField, val: u64) -> Result<(), x86::vmx::VmFail> {
    unsafe { x86::bits64::vmx::vmwrite(field as u32, val) }
}

/// Read the contents of the current machine's dr7 (debug register 7).
pub fn read_dr7() -> u64 {
    let ret: u64;
    unsafe {
        asm!("mov {}, dr7", out(reg)(ret));
    }
    ret
}

/// Returns true if the Intel vmx extensions are available and a hypervisor is not present, false otherwise.
fn vmx_available() -> bool {
    let result = unsafe { core::arch::x86_64::__cpuid(CPUIDLeaf::ProcessorInfoAndFeatures as u32) };
    result.ecx & (CPUIDLeafProcessorInfoAndFeaturesECXBits::VMXAvailable as u32) != 0
}

/// Gets the current VMCS revision identifier.
/// This is used to initialize the vmxon region and the vmcs.
fn get_vmcs_revision_identifier() -> u32 {
    let pair = rdmsr(Msr::Ia32VmxBasic);
    let vmcs_revision_identifier = pair.eax;
    assert!((vmcs_revision_identifier & (1 << 31)) == 0);
    vmcs_revision_identifier
}

fn set_cr0_bits() {
    let fixed0 = rdmsrl(Msr::Ia32VmxCr0Fixed0);
    let fixed1 = rdmsrl(Msr::Ia32VmxCr0Fixed1);
    let mut cr0 = unsafe { x86::controlregs::cr0() };
    cr0 |= x86::controlregs::Cr0::from_bits_truncate(fixed0 as usize);
    cr0 &= x86::controlregs::Cr0::from_bits_truncate(fixed1 as usize);
    unsafe {
        x86::controlregs::cr0_write(cr0);
    }
}

fn set_cr4_bits() {
    let fixed0 = rdmsrl(Msr::Ia32VmxCr4Fixed0);
    let fixed1 = rdmsrl(Msr::Ia32VmxCr4Fixed1);
    let mut cr4 = unsafe { x86::controlregs::cr4() };
    cr4 |= x86::controlregs::Cr4::from_bits_truncate(fixed0 as usize);
    cr4 &= x86::controlregs::Cr4::from_bits_truncate(fixed1 as usize);
    unsafe {
        x86::controlregs::cr4_write(cr4);
    }
}

fn set_lock_bit() -> Result<(), ()> {
    let mut pair = rdmsr(Msr::Ia32FeatureControl);
    if (pair.eax & IA32_FEATURE_CONTROL_LOCK_BIT) == 0 {
        info!("Setting lock bit");
        pair.eax |=
            IA32_FEATURE_CONTROL_VMX_ENABLED_OUTSIDE_SMX_BIT | IA32_FEATURE_CONTROL_LOCK_BIT;
        wrmsr(Msr::Ia32FeatureControl, pair);
        Ok(())
    } else if (pair.eax & IA32_FEATURE_CONTROL_VMX_ENABLED_OUTSIDE_SMX_BIT) == 0 {
        error!("Lock bit is set but vmx is disabled. Hypervisor cannot start");
        Err(())
    } else {
        Ok(())
    }
}

fn prepare_vmx_memory_region(vmx_region: *mut u32, vmx_region_size: usize) {
    assert!(!vmx_region.is_null());
    assert!(vmx_region_size <= 0x1000);
    assert!(vmx_region_size > mem::size_of::<u32>());

    unsafe {
        ptr::write_bytes(vmx_region, 0, vmx_region_size / core::mem::size_of::<u32>());
        ptr::write(vmx_region, get_vmcs_revision_identifier());
        trace!("Setting vmxon region identifier {:x}", *vmx_region);
    }
}

pub enum VmxEnablementError {
    VmxExtensionsAbsent,
    InvalidVmxOnRegion,
    LockBitSetButVmxDisabled,
    VmxOnFailure,
}

pub fn enable(
    vmxon_region: *mut u32,
    vmxon_region_phys: u64,
    vmxon_region_size: usize,
) -> Result<(), VmxEnablementError> {
    assert!(is_page_aligned(vmxon_region as u64));
    assert!(is_page_aligned(vmxon_region_phys));

    if vmxon_region.is_null() {
        error!("Bad VMX on region");
        return Err(VmxEnablementError::VmxExtensionsAbsent);
    }

    if !vmx_available() {
        error!("VMX unavailable");
        return Err(VmxEnablementError::InvalidVmxOnRegion);
    }

    trace!("Setting lock bit");
    set_lock_bit().map_err(|_| {
        error!("Lock bit not set");
        VmxEnablementError::LockBitSetButVmxDisabled
    })?;

    trace!("Setting cr0 bits");
    set_cr0_bits();
    trace!("Setting cr4 bits");
    set_cr4_bits();

    trace!("Preparing vmxon region");
    prepare_vmx_memory_region(vmxon_region, vmxon_region_size);

    trace!("Doing vmxon");
    match unsafe { x86::bits64::vmx::vmxon(vmxon_region_phys) } {
        Ok(()) => {
            trace!("vmxon succeeded");
            Ok(())
        }
        Err(e) => {
            error!("vmxon failed {:x?}", e);
            Err(VmxEnablementError::VmxOnFailure)
        }
    }
}

pub fn disable() {
    unimplemented!();
}

extern "C" {
    fn _guest_first_entry() -> usize;
}

pub fn load_vm(vcpu: &VCpu) -> Result<(), x86::vmx::VmFail> {
    trace!(
        "Loading vmm with vcpu {:x?} {:x?}",
        vcpu,
        vcpu as *const VCpu
    );
    assert!(is_page_aligned(vcpu.vmcs as u64));
    assert!(is_page_aligned(vcpu.vmcs_phys));

    trace!("Preparing vmcs");
    prepare_vmx_memory_region(vcpu.vmcs, vcpu.vmcs_size);

    trace!("vmclear");
    unsafe {
        x86::bits64::vmx::vmclear(vcpu.vmcs_phys)?;
    }

    trace!("vmptrld");
    unsafe {
        x86::bits64::vmx::vmptrld(vcpu.vmcs_phys)?;
    }

    trace!("Initializing vm control values ");
    vmcs::initialize_vm_control_values(vcpu)?;
    trace!("Initializing host state");
    vmcs::initialize_host_state(vcpu)?;
    trace!("Initializing guest state");
    vmcs::initialize_guest_state(vcpu)?;

    trace!("Launching...");

    crate::vmcs_dump::dump();
    let guest_first_entry_result = unsafe { _guest_first_entry() };

    match guest_first_entry_result {
        0 => {
            //trace!("Successfully entered the guest");
            Ok(())
        }
        1 => {
            trace!("vmfailvalid");
            Err(x86::vmx::VmFail::VmFailValid)
        }
        2 => {
            trace!("vmfailinvalid");
            Err(x86::vmx::VmFail::VmFailInvalid)
        }
        other => {
            trace!("unknown guest entry code {:x}", other);
            Err(x86::vmx::VmFail::VmFailInvalid)
        }
    }
}

pub fn unload_vm() {
    /*
    if let Ok(vmcs_phys) = vmptrst() {
        if let Err(code) = vmclear(vmcs_phys) {
            error!("vmclear failed with error code {}", code);
        }
    }
    */
    unimplemented!();
}
