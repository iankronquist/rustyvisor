#![no_std]
extern crate hypervisor;
use core::convert::TryFrom;

const PAGE_SIZE: usize = 0x1000;
extern "C" {
    fn rustyvisor_linux_virt_to_phys(x: *mut u8) -> u64;
    fn rustyvisor_linux_kmalloc(size: usize) -> *mut u8;
}
use hypervisor::segmentation::{GdtEntry, GdtEntry64};

use hypervisor::segmentation::Tss;

fn rustyvisor_linux_allocate_vcpu() -> Result<&'static mut hypervisor::VCpu, ()> {
    unsafe {
        let vcpu = rustyvisor_linux_kmalloc(core::mem::size_of::<hypervisor::VCpu>())
            as *mut hypervisor::VCpu;
        if vcpu.is_null() {
            return Err(());
        }

        let tss = rustyvisor_linux_kmalloc(core::mem::size_of::<hypervisor::segmentation::Tss>());
        if tss.is_null() {
            return Err(());
        }

        let virtual_local_interrupt_controller = rustyvisor_linux_kmalloc(core::mem::size_of::<
            hypervisor::interrupt_controller::VirtualLocalInterruptController,
        >())
            as *mut hypervisor::interrupt_controller::VirtualLocalInterruptController;
        if virtual_local_interrupt_controller.is_null() {
            return Err(());
        }

        let vmxon_region = rustyvisor_linux_kmalloc(PAGE_SIZE) as *mut u32;
        if vmxon_region.is_null() {
            return Err(());
        }
        let vmx_on_region_phys = rustyvisor_linux_virt_to_phys(vmxon_region as *mut u8);


        let vmcs = rustyvisor_linux_kmalloc(PAGE_SIZE) as *mut u32;
        if vmcs.is_null() {
            return Err(());
        }
        let vmcs_phys = rustyvisor_linux_virt_to_phys(vmcs as *mut u8);

        let stack_pages = 1;
        let stack = rustyvisor_linux_kmalloc(stack_pages * PAGE_SIZE);
        if stack.is_null() {
            return Err(());
        }

        let msr_bitmap = rustyvisor_linux_kmalloc(PAGE_SIZE);
        if msr_bitmap.is_null() {
            return Err(());
        }
        let msr_bitmap_phys = rustyvisor_linux_virt_to_phys(msr_bitmap);


        let gdt = hypervisor::segmentation::get_current_gdt();
        let original_gdt_size = gdt.len() * core::mem::size_of::<GdtEntry>();
        let host_gdt_size = core::mem::size_of_val(&gdt) + core::mem::size_of::<GdtEntry64>();
        let host_tr_index = gdt.len();
        let host_gdt = rustyvisor_linux_kmalloc(host_gdt_size);
        if host_gdt.is_null() {
            return Err(());
        }

        (*vcpu).this_vcpu = vcpu;

        (*vcpu).loaded_successfully = false;

        (*vcpu).vmcs_phys = vmcs_phys;
        (*vcpu).vmcs_size = PAGE_SIZE;
        (*vcpu).vmcs = vmcs;

        (*vcpu).virtual_local_interrupt_controller = virtual_local_interrupt_controller;

        (*vcpu).vmxon_region_phys = vmx_on_region_phys;
        (*vcpu).vmxon_region_size = PAGE_SIZE;
        (*vcpu).vmxon_region = vmxon_region;

        (*vcpu).msr_bitmap = msr_bitmap_phys;
        (*vcpu).stack_base = stack;
        (*vcpu).stack_size = stack_pages * PAGE_SIZE; // Page size
        (*vcpu).stack_top = (*vcpu).stack_base.add((*vcpu).stack_size);

        (*vcpu).host_gdt_base = host_gdt as *mut u64;
        (*vcpu).host_gdt_limit = host_gdt_size as u64 - 1;

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
        Ok(&mut *vcpu)
    }
}

#[no_mangle]
pub extern "C" fn rustyvisor_linux_core_load(_ptr: usize) -> i32 {
    let vcpu = match rustyvisor_linux_allocate_vcpu() {
        Ok(vcpu) => vcpu,
        Err(_) => return -1,
    };
    hypervisor::rustyvisor_core_load(vcpu)
}
