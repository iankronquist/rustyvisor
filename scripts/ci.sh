#!/bin/bash

set -x
set -e

./build.sh

#cargo clippy -- -W clippy::all --target x86_64-unknown-uefi
cargo fmt --all -- --check
