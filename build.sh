#!/bin/sh

set -x
set -e


OS="`uname`"

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

if [ "$OS" = "Linux" ]; then

cd linux || exit 255
make
cd ..

else

echo "On a non-Linux OS. I will only build the rust parts of the Linux Kernel Module version of RustyVisor"
cd linux || exit 255
make rust-only
cd ..

fi
