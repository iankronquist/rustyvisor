# RustyVisor

A hypervisor written in Rust.

The goal of this project is to learn more Rust and determine whether Rust is
a significant improvement over C for systems programming tasks like
implementing hypervisors.

# Building

Depends on nightly Rust, Xargo, GCC, and the Linux module development headers.

```
$ ./scripts/remove-kvm.sh
$ make
$ sudo insmod rustyvisor.ko
$ sudo rmmod rustyvisor.ko
```
