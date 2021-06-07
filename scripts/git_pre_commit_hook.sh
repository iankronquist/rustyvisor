#!/bin/bash
cargo fmt -- --check
if [[ $? -ne 0 ]] ; then
    echo Please run cargo fmt before committing.
    exit 1
fi
exit 0
