//! A UEFI executable for communicating with the
//! [hypervisor](../hypervisor/index.html).
//! Invoke via the UEFI shell like so:
//! ```text
//! UEFI Interactive Shell v2.2
//! EDK II
//! UEFI v2.70 (EDK II, 0x00010000)
//! Mapping table
//!       FS0: Alias(s):F0a:;BLK0:
//!           PciRoot(0x0)/Pci(0x1,0x1)/Ata(0x0)
//! Press ESC in 1 seconds to skip startup.nsh or any other key to continue.
//! Shell> fs0:
//! FS0:\> dir
//! Directory of: FS0:\
//! 06/03/2021  23:33             342,016  rustyvctl.efi
//! 06/03/2021  23:42              10,383  NvVars
//!           2 File(s)     433,807 bytes
//!           0 Dir(s)
//! FS0:\> .\rustyvctl.efi
//! FS0:\>
//! ```

#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![warn(missing_docs)]

extern crate hypervisor;
extern crate uefi;
use uefi::prelude::*;

use core::ffi::c_void;

use core::convert::TryFrom;

/// The entrypoint of the UEFI runtime service.
/// Sets up the hypervisor and loads it on every core using the UEFI
/// multi-processing protocol.
#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: uefi::Handle,
    system_table: SystemTable<Boot>,
) -> Status {
    Status::SUCCESS
}
