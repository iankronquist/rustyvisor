#!/bin/bash

set -x
set -e

if [[ ! -e $HOME/.cargo/bin/rustup ]]; then
  curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=$TRAVIS_RUST_VERSION -y;
  cargo install clippy --force;
fi

source ~/.cargo/env

rustup install nightly

rustup default nightly

rustup component add rust-src

rustup component add clippy

rustup component add rustfmt

sudo apt-get install linux-headers-$(uname -r)

cargo install xargo || true

cargo install rustfmt || true
