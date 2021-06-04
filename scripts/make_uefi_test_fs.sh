#!/bin/sh

dd if=/dev/zero of=fat.img bs=1024 count=16384
mkfs.vfat fat.img
