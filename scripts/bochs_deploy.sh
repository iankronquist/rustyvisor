#!/bin/sh
mcopy -oi fat.img target/x86_64-unknown-uefi/debug/rustyvisor.efi ::
mcopy -oi fat.img target/x86_64-unknown-uefi/debug/rustyvctl.efi ::
mcopy -oi fat.img scripts/startup.nsh ::
