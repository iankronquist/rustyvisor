#!/bin/bash

set -x
set -e

# Deny warnings on all rust code.
export RUSTFLAGS="-D warnings"

./build.sh

#cargo clippy -- -W clippy::all --target x86_64-unknown-uefi
cargo fmt --all -- --check
