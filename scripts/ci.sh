#!/bin/bash

set -x
set -e

make
make test
cargo clippy -- -D clippy || true
cargo fmt -- --write-mode diff
dmesg -w &
sudo insmod rustyvisor.ko
sudo rmmod rustyvisor.ko
