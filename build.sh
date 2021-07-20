#!/bin/sh

set -x
set -e

if [ "$1" = "clean" ] ; then
    cd uefi || exit 255
    cargo clean
    cd ..

    cd rustyvctl || exit 255
    cargo clean
    cd ..

    cd linux || exit 255
    make clean
    cd ..

    exit 0
fi

cd uefi || exit 255
cargo build
cd ..

cd rustyvctl || exit 255
cargo build
cd ..

cd linux || exit 255
make
cd ..
