Changes:
 - `Makefile`: [add](https://stackoverflow.com/questions/48040146/error-loading-target-specification-could-not-find-specification-for-target) `RUST_TARGET_PATH` for xargo
 - `Xargo.toml`: change `collections` to `alloc` because it doesn't exist [no more](https://github.com/serde-rs/serde/issues/955)
 - `src/runtime.rs`: change `panic_fmt` to `panic_handler` because a [breaking-change](https://users.rust-lang.org/t/psa-breaking-change-panic-fmt-language-item-removed-in-favor-of-panic-implementation/17875)
 - `src/lib.rs`: remove `#![feature(use_extern_macros)]`
 - `x86_64-linux.json`: avoid relocation type `R_X86_64_GOTPCREL` because of dmesg log [unknown rela relocation 9](https://github.com/rust-lang/rust/issues/57390)
