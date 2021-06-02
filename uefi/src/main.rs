#![no_std]
#![no_main]
#![feature(abi_efiapi)]

extern crate uefi;
extern crate uefi_services;
extern crate hypervisor;

use uefi::prelude::*;

#[entry]
fn efi_main(_image_handle: uefi::Handle, _system_table: SystemTable<Boot>) -> Status {
    hypervisor::rustyvisor_load();
    loop {}
            //Status::SUCCESS
}
