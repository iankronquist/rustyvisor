#!/bin/bash

set -x
set -e

source ~/.cargo/env

# Install Linux Kernel headers
sudo apt-get install linux-headers-$(uname -r)

# Install UEFI Rust target
rustup target install x86_64-unknown-uefi
# Install Rust source code
rustup component add rust-src
