#!/bin/bash

set -x
set -e

cargo +nightly build --target x86_64-unknown-uefi

#cargo clippy -- -W clippy::all --target x86_64-unknown-uefi
cargo fmt -- --check
