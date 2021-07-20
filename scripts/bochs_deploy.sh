#!/bin/sh
mcopy -i fat.img target/x86_64-unknown-uefi/debug/rustyvisor.efi ::
mcopy -i fat.img target/x86_64-unknown-uefi/debug/rustyvctl.efi ::
