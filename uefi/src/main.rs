#![no_std]
#![no_main]
#![feature(abi_efiapi)]

extern crate uefi;
extern crate uefi_services;
extern crate hypervisor;

use core::ffi::c_void;


use core::{convert::TryFrom, convert::TryInto, usize};

use hypervisor::segmentation::GdtEntry;

use uefi::proto::pi::mp::MpServices;
use uefi::{prelude::*, table::boot::MemoryType};

fn efi_phys_to_virt<T>(phys: u64) -> *mut T {
    phys as *mut T
}

const PAGE_SIZE: usize = 0x1000;

// FIXME should zero all of this
fn efi_create_vcpu(system_table: &SystemTable<Boot>) -> uefi::Result<*mut hypervisor::VCpu> {
    let vcpu = system_table.boot_services().allocate_pool(MemoryType::RUNTIME_SERVICES_DATA, core::mem::size_of::<hypervisor::VCpu>())?.expect("Allocation completed") as *mut hypervisor::VCpu;

    let tss = system_table.boot_services().allocate_pool(MemoryType::RUNTIME_SERVICES_DATA, core::mem::size_of::<hypervisor::segmentation::Tss>())?.expect("Allocation completed") as *mut hypervisor::segmentation::Tss;


    let vmx_on_region_phys= system_table.boot_services().allocate_pages(uefi::table::boot::AllocateType::AnyPages, MemoryType::RUNTIME_SERVICES_DATA, 1)?;
    let vmcs_phys= system_table.boot_services().allocate_pages(uefi::table::boot::AllocateType::AnyPages, MemoryType::RUNTIME_SERVICES_DATA, 1)?;

    let stack_pages = 1;
    let stack= system_table.boot_services().allocate_pages(uefi::table::boot::AllocateType::AnyPages, MemoryType::RUNTIME_SERVICES_DATA, stack_pages)?;

    let gdt = hypervisor::segmentation::get_current_gdt();
    let original_gdt_size = gdt.len() * core::mem::size_of::<GdtEntry>();
    let host_gdt_size = (gdt.len() + 1) * core::mem::size_of::<GdtEntry>();
    let host_tr_index = gdt.len();
    let host_gdt = system_table.boot_services().allocate_pool(MemoryType::RUNTIME_SERVICES_DATA, host_gdt_size)?.expect("Completion failed?");

    unsafe {
        system_table.boot_services().memmove(host_gdt, &gdt as *const _ as *const u8, original_gdt_size);

        (*vcpu).loaded_successfully = false;

        (*vcpu).vmcs_phys = vmcs_phys.expect("vmcs allocation");
        (*vcpu).vmcs_size = PAGE_SIZE;
        (*vcpu).vmcs = efi_phys_to_virt((*vcpu).vmcs_phys);
        
        (*vcpu).vmxon_region_phys = vmx_on_region_phys.expect("vmx on allocation");
        (*vcpu).vmxon_region_size = PAGE_SIZE;
        (*vcpu).vmxon_region = efi_phys_to_virt((*vcpu).vmxon_region_phys);

        (*vcpu).stack_base = efi_phys_to_virt(stack.expect("Stack"));
        (*vcpu).stack_size = stack_pages * PAGE_SIZE; // Page size
        (*vcpu).stack_top = (*vcpu).stack_base.add((*vcpu).stack_size);

        (*vcpu).host_gdt_base = host_gdt as *mut u64;
        (*vcpu).host_gdt_limit = host_gdt_size as u64 - 1;

        (*vcpu).tr_base = tss as u64;
        (*vcpu).tr_selector = u16::try_from(host_tr_index * core::mem::size_of::<GdtEntry>()).unwrap();
    };

    Ok(uefi::Completion::new(Status::SUCCESS, vcpu))
}

extern "efiapi" fn efi_core_load(arg: *mut c_void) {
    let system_table = unsafe { &*(arg as *const SystemTable<Boot>)};
    let vcpu = unsafe { &*efi_create_vcpu(system_table).unwrap().unwrap()};
    unsafe {
        hypervisor::rustyvisor_core_load(vcpu);
    }

}

#[entry]
fn efi_main(_image_handle: uefi::Handle, system_table: SystemTable<Boot>) -> Status {
    hypervisor::rustyvisor_load();
    let vcpu = unsafe { &*efi_create_vcpu(&system_table).unwrap().unwrap()};
    unsafe {
    hypervisor::rustyvisor_core_load(vcpu);
    }

    let mp_proto = system_table.boot_services().locate_protocol::<MpServices>().expect("Mp services not found").expect("Completion failure");
    let mp_proto = unsafe { &mut *mp_proto.get() };

    mp_proto.startup_all_aps(false, efi_core_load, &system_table as *const SystemTable<Boot> as *mut c_void, None)
    .unwrap()
    .unwrap();

    Status::SUCCESS
}
