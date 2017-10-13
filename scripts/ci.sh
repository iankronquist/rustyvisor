#!/bin/bash

set -x
set -e

make
make test
cargo clippy -- -D clippy || true
cargo fmt -- --write-mode diff
bash /home/travis/build/iankronquist/rustyvisor/scripts/printer.sh
dmesg
sudo insmod rustyvisor.ko
dmesg
sudo rmmod rustyvisor.ko
