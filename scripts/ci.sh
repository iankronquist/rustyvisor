#!/bin/bash

set -x
set -e

make
make test
cargo clippy -- -W clippy::all
cargo fmt -- --check
