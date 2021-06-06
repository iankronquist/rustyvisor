cargo +nightly build --target x86_64-unknown-uefi -Z build-std=core \
        -Z build-std-features=compiler-builtins-mem
