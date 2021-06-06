#![no_std]
#![no_main]
#![feature(abi_efiapi)]

extern crate hypervisor;
extern crate uefi;

use core::ffi::c_void;

use core::convert::TryFrom;

use hypervisor::segmentation;
use hypervisor::segmentation::{GdtEntry, GdtEntry64};

use hypervisor::segmentation::Tss;
use uefi::proto::pi::mp::MpServices;
use uefi::{prelude::*, table::boot::MemoryType};

fn efi_phys_to_virt<T>(phys: u64) -> *mut T {
    phys as *mut T
}

extern "C" {
    static __ImageBase: u8;
}

const PAGE_SIZE: usize = 0x1000;

fn efi_create_vcpu(system_table: &SystemTable<Boot>) -> uefi::Result<*mut hypervisor::VCpu> {
    let vcpu = system_table
        .boot_services()
        .allocate_pool(
            MemoryType::RUNTIME_SERVICES_DATA,
            core::mem::size_of::<hypervisor::VCpu>(),
        )?
        .expect("Allocation completed") as *mut hypervisor::VCpu;

    let tss = system_table
        .boot_services()
        .allocate_pool(
            MemoryType::RUNTIME_SERVICES_DATA,
            core::mem::size_of::<hypervisor::segmentation::Tss>(),
        )?
        .expect("Allocation completed");

    let virtual_local_interrupt_controller = system_table
        .boot_services()
        .allocate_pool(
            MemoryType::RUNTIME_SERVICES_DATA,
            core::mem::size_of::<hypervisor::interrupt_controller::VirtualLocalInterruptController>(
            ),
        )?
        .expect("Allocation completed")
        as *mut hypervisor::interrupt_controller::VirtualLocalInterruptController;

    let vmx_on_region_phys = system_table.boot_services().allocate_pages(
        uefi::table::boot::AllocateType::AnyPages,
        MemoryType::RUNTIME_SERVICES_DATA,
        1,
    )?;
    let vmcs_phys = system_table.boot_services().allocate_pages(
        uefi::table::boot::AllocateType::AnyPages,
        MemoryType::RUNTIME_SERVICES_DATA,
        1,
    )?;

    let stack_pages = 1;
    let stack = system_table.boot_services().allocate_pages(
        uefi::table::boot::AllocateType::AnyPages,
        MemoryType::RUNTIME_SERVICES_DATA,
        stack_pages,
    )?;

    let msr_bitmap = system_table
        .boot_services()
        .allocate_pages(
            uefi::table::boot::AllocateType::AnyPages,
            MemoryType::RUNTIME_SERVICES_DATA,
            1,
        )?
        .expect("msr bitmap allocated");

    let gdt = hypervisor::segmentation::get_current_gdt();
    let original_gdt_size = gdt.len() * core::mem::size_of::<GdtEntry>();
    let host_gdt_size = core::mem::size_of_val(&gdt) + core::mem::size_of::<GdtEntry64>();
    let host_tr_index = gdt.len();
    let host_gdt = system_table
        .boot_services()
        .allocate_pool(MemoryType::RUNTIME_SERVICES_DATA, host_gdt_size)?
        .expect("Completion failed?");

    unsafe {
        (*vcpu).this_vcpu = vcpu;

        system_table.boot_services().memmove(
            host_gdt,
            &gdt as *const _ as *const u8,
            original_gdt_size,
        );

        (*vcpu).loaded_successfully = false;

        (*vcpu).vmcs_phys = vmcs_phys.expect("vmcs allocation");
        (*vcpu).vmcs_size = PAGE_SIZE;
        (*vcpu).vmcs = efi_phys_to_virt((*vcpu).vmcs_phys);

        (*vcpu).virtual_local_interrupt_controller = virtual_local_interrupt_controller;
        system_table.boot_services().memset(
            (*vcpu).virtual_local_interrupt_controller as *mut u8,
            core::mem::size_of::<hypervisor::interrupt_controller::VirtualLocalInterruptController>(
            ),
            0,
        );

        system_table
            .boot_services()
            .memset((*vcpu).vmcs as *mut u8, (*vcpu).vmcs_size, 0);

        (*vcpu).vmxon_region_phys = vmx_on_region_phys.expect("vmx on allocation");
        (*vcpu).vmxon_region_size = PAGE_SIZE;
        (*vcpu).vmxon_region = efi_phys_to_virt((*vcpu).vmxon_region_phys);

        system_table.boot_services().memset(
            (*vcpu).vmxon_region as *mut u8,
            (*vcpu).vmxon_region_size,
            0,
        );

        (*vcpu).msr_bitmap = msr_bitmap;
        system_table
            .boot_services()
            .memset(efi_phys_to_virt((*vcpu).msr_bitmap), PAGE_SIZE, 0);

        (*vcpu).stack_base = efi_phys_to_virt(stack.expect("Stack"));
        (*vcpu).stack_size = stack_pages * PAGE_SIZE; // Page size
        (*vcpu).stack_top = (*vcpu).stack_base.add((*vcpu).stack_size);

        (*vcpu).host_gdt_base = host_gdt as *mut u64;
        (*vcpu).host_gdt_limit = host_gdt_size as u64 - 1;

        system_table
            .boot_services()
            .memset(tss, core::mem::size_of::<segmentation::Tss>(), 0);
        let tss_base = tss as u64;
        (*vcpu).tr_base = tss_base;
        (*vcpu).tr_selector =
            u16::try_from(host_tr_index * core::mem::size_of::<GdtEntry>()).unwrap();
        let tss_gdt_entry = host_gdt.add(original_gdt_size) as *mut GdtEntry64;
        (*tss_gdt_entry).access = 0xe9;
        (*tss_gdt_entry).granularity = 0;
        (*tss_gdt_entry).limit_low = u16::try_from(core::mem::size_of::<Tss>() - 1).unwrap();
        (*tss_gdt_entry).base_low = tss_base as u16;
        (*tss_gdt_entry).base_middle = (tss_base >> 16) as u8;
        (*tss_gdt_entry).base_high = (tss_base >> 24) as u8;
        (*tss_gdt_entry).base_highest = (tss_base >> 32) as u32;
        (*tss_gdt_entry).reserved0 = 0;

    };

    Ok(uefi::Completion::new(Status::SUCCESS, vcpu))
}

extern "efiapi" fn efi_core_load(arg: *mut c_void) {
    let system_table = unsafe { &*(arg as *const SystemTable<Boot>) };
    let vcpu_result = efi_create_vcpu(system_table);
    let vcpu_ptr = vcpu_result.unwrap().unwrap();
    let vcpu = unsafe { &*vcpu_ptr };
    hypervisor::rustyvisor_core_load(vcpu);
}

#[entry]
fn efi_main(_image_handle: uefi::Handle, system_table: SystemTable<Boot>) -> Status {
    hypervisor::rustyvisor_load();

    efi_core_load(&system_table as *const SystemTable<Boot> as *mut c_void);

    let mp_proto = system_table
        .boot_services()
        .locate_protocol::<MpServices>()
        .expect("Mp services not found")
        .expect("Completion failure");
    let mp_proto = unsafe { &mut *mp_proto.get() };


    match mp_proto.startup_all_aps(
        false,
        efi_core_load,
        &system_table as *const SystemTable<Boot> as *mut c_void,
        None,
    ) {
        Ok(_) => {
            Status::SUCCESS
        }
        Err(e) => match e.status() {
            Status::NOT_STARTED => {
                Status::SUCCESS
            }
            e => {
                e
            }
        },
    }
}
