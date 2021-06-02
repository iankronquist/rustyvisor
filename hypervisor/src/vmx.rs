use ::log::{error, info, log};

use core::{mem, ptr};

use crate::msr::{rdmsrl, rdmsr, wrmsr, Msr};
use crate::{PerCoreData, vmcs};
use crate::vmcs_fields::VmcsField;




const IA32_FEATURE_CONTROL_LOCK_BIT: u32 = 1 << 0;
const IA32_FEATURE_CONTROL_VMX_ENABLED_OUTSIDE_SMX_BIT: u32 = 1 << 2;

#[repr(u32)]
pub enum CPUIDLeaf {
    ProcessorInfoAndFeatures = 1,
}

#[repr(u32)]
pub enum CPUIDLeafProcessorInfoAndFeaturesECXBits {
    VMXAvailable = 1 << 5,
    HypervisorPresent = 1 << 31,
}

pub const fn is_page_aligned(n: u64) -> bool {
    n.trailing_zeros() >= 12
}

pub fn vmxon(addr: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor eax, eax; \
         vmxon [{vmxon_region_phys}]; \
         setc ah; \
         setz al;",
        vmxon_region_phys = in(reg) addr,
        lateout("eax")(ret),
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
        );
    }
}

pub fn vmread(field: VmcsField) -> Result<u64, u32> {
    let ret: u32;
    let val: u64;
    unsafe {
        asm!(
        "xor eax, eax; \
         vmread {value}, {field}; \
         setc ah; \
         setz al;",
         value = out(reg) (val),
         field = in(reg) (field as u64),
         lateout("eax")(ret),
        );
    }
    if ret == 0 {
        Ok(val)
    } else {
        Err(ret)
    }
}

pub fn vmwrite(field: VmcsField, val: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor eax, eax; \
         vmwrite {value}, {field}; \
         setc ah; \
         setz al;",
         value = in(reg) (val),
         field = in(reg) (field as u64),
         lateout("eax")(ret),
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}


pub fn vmptrld(vmcs_phys: u64) -> Result<(), u32> {
    let ret: u32;
    unsafe {
        asm!(
        "xor eax, eax; \
        vmptrld [{vmcs_phys}]; \
         setc ah; \
         setz al;",
         vmcs_phys = in(reg) vmcs_phys,
        lateout("eax")(ret),
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
        "xor eax, eax; \
        vmclear [{vmcs_phys}]; \
         setc ah; \
         setz al;",
         vmcs_phys = in(reg) vmcs_phys,
        lateout("eax")(ret),
        );
    }
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn vmptrst() -> Result<u64, u32> {
    let ret: u32;
    let vmcs_phys: u64 = 0;
    unsafe {
        asm!(
        "xor eax, eax; \
        vmptrst [{vmcs_phys}]; \
         setc ah; \
         setz al;",
         vmcs_phys = in(reg) vmcs_phys,
        lateout("eax")(ret),
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
        "xor eax, eax; \
        vmlaunch; \
         setc ah; \
         setz al;",
        lateout("eax")(ret),
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
            "xor eax, eax; \
            vmlaunch; \
             setc ah; \
             setz al;",
            lateout("eax")(ret),
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
        "mov ax, cs",
        lateout("eax") (ret)
        );
    }
    ret
}

pub fn read_ds() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov ax, ds",
        lateout("eax") (ret)
        );
    }
    ret
}

pub fn read_es() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov ax, es",
        lateout("eax") (ret)
        );
    }
    ret
}

pub fn read_fs() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov ax, fs",
        lateout("eax") (ret)
        );
    }
    ret
}

pub fn read_gs() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov ax, gs",
        lateout("eax") (ret)
        );
    }
    ret
}

pub fn read_ss() -> u16 {
    let ret: u16;
    unsafe {
        asm!(
        "mov ax, ss",
        lateout("eax") (ret)
        );
    }
    ret
}

pub fn read_cr3() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "mov {}, cr3",
        lateout(reg) (ret)
        );
    }
    ret
}

pub fn read_cr4() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "mov {}, cr4",
        lateout(reg) (ret)
        );
    }
    ret
}

pub fn read_cr0() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "mov {}, cr0",
        lateout(reg) (ret)
        );
    }
    ret
}


pub fn write_cr0(val: u64) {
    unsafe {
        asm!(
        "mov cr0, {}",
        in(reg) (val)
        );
    }
}

pub fn write_cr4(val: u64) {
    unsafe {
        asm!(
        "mov cr4, {}",
        in(reg) (val)
        );
    }
}


pub fn read_db7() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
            "mov db7, {}",
            lateout(reg) (ret)
        );
    }
    ret
}

pub fn read_flags() -> u64 {
    let ret: u64;
    unsafe {
        asm!(
        "pushf; pop {}",
        lateout(reg) (ret)
        );
    }
    ret
}

fn vmx_available() -> bool {
    let result = unsafe { core::arch::x86_64::__cpuid(CPUIDLeaf::ProcessorInfoAndFeatures as u32) };
    result.ecx & (CPUIDLeafProcessorInfoAndFeaturesECXBits::VMXAvailable as u32) != 0
}

fn get_vmcs_revision_identifier() -> u32 {
    let (_high_bits, vmcs_revision_identifier) = rdmsr(Msr::Ia32VmxBasic);
    assert!((vmcs_revision_identifier & (1 << 31)) == 0);
    vmcs_revision_identifier
}


fn set_cr0_bits() {
    let fixed0 = rdmsrl(Msr::Ia32VmxCr0Fixed0);
    let fixed1 = rdmsrl(Msr::Ia32VmxCr0Fixed1);
    let mut cr0 = read_cr0();
    cr0 |= fixed0;
    cr0 &= fixed1;
    write_cr0(cr0);
}

fn set_cr4_bits() {
    let fixed0 = rdmsrl(Msr::Ia32VmxCr4Fixed0);
    let fixed1 = rdmsrl(Msr::Ia32VmxCr4Fixed1);
    let mut cr4 = read_cr4();
    cr4 |= fixed0;
    cr4 &= fixed1;
    write_cr4(cr4);
}

fn set_lock_bit() -> Result<(), ()> {
    let (_high, low) = rdmsr(Msr::Ia32FeatureControl);
    if (low & IA32_FEATURE_CONTROL_LOCK_BIT) == 0 {
        wrmsr(
            Msr::Ia32FeatureControl,
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

fn prepare_vmx_memory_region(vmx_region: *mut u32, vmx_region_size: usize) {
    assert!(!vmx_region.is_null());
    assert!(vmx_region_size <= 0x1000);
    assert!(vmx_region_size > mem::size_of::<u32>());

    unsafe {
        ptr::write_bytes(vmx_region, 0, vmx_region_size);
        ptr::write(vmx_region, get_vmcs_revision_identifier());
    }
}

pub fn enable(
    vmxon_region: *mut u32,
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
    if result == Ok(()) {
        info!("vmxon succeeded");
        Ok(())
    } else {
        error!("vmxon failed");
        Err(())
    }
}


pub fn disable() {
    vmxoff();
    info!("vmxoff");
}

pub fn load_vm(vcpu: &PerCoreData) -> Result<(), u32> {
    assert!(is_page_aligned(vcpu.vmcs as u64));
    assert!(is_page_aligned(vcpu.vmcs_phys));

    prepare_vmx_memory_region(vcpu.vmcs, vcpu.vmcs_size);

    vmptrld(vcpu.vmcs_phys)?;
    vmclear(vcpu.vmcs_phys)?;

    vmcs::initialize_host_state(vcpu)?;
    vmcs::initialize_guest_state();
    vmcs::initialize_vm_control_values()?;

    vmlaunch().or_else(|_| {
        match vmread(VmcsField::VmInstructionError) {
            Ok(vm_instruction_error_number) => error!(
                "Failed to launch VM because {} ({})",
                vmcs::vm_instruction_error_number_message(vm_instruction_error_number),
                vm_instruction_error_number
            ),
            Err(e) => error!("VMLaunch failed with {}", e),
        }
        Err(!0u32)
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
