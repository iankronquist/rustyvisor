#!/bin/bash

set -x
set -e

make
make test
cargo clippy -- -D clippy || true
cargo fmt -- --write-mode diff
dmesg -w &
insmod rustyvisor.ko
rmmod rustyvisor.ko
