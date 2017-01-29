# RustyVisor

A hypervisor written in Rust.

The goal of this project is to learn more Rust and determine whether Rust is
a significant improvement over C for systems programming tasks like
implementing hypervisors.

This project takes the form of an Linux Kernel module which loads an x86_64
type II hypervisor which virtualizes the original host operating system. After
inserting the module, Linux will be running inside a VM as a guest, and the
host operating system will be RustyVisor!


## Installing dependencies

Depends on nightly Rust, Xargo, GCC, and the Linux module development headers.
Oh Ubuntu this should be as simple as:

```
$ # Bootstrap rustup to get a nightly build of Rust.
$ curl https://sh.rustup.rs -sSf | sh
$ source ~/.cargo/env
$ rustup install nightly
$ rustup default nightly
$ # Install development headers & GCC
$ sudo apt-get install linux-headers-$(uname -r) gcc # ubuntu
$ sudo dnf install kernel-devel-$(uname -r) # fedora
$ # The cross compilation tool Xargo needs the rust source code
$ rustup component add rust-src
$ # Install Xargo
$ cargo install xargo
```

## Building the hypervisor

```
$ make
```

## Running the hypervisor
Your Linux kernel may already have KVM installed. If so, we have to remove it
first.

```
$ ./scripts/remove-kvm.sh
$ sudo insmod rustyvisor.ko
$ sudo rmmod rustyvisor.ko
```

## Contributions & Bugs

Pull requests welcome! This is a project in part to learn about Rust, so if
there is code which isn't idiomatic, please feel free to show me a better way.

Please file bugs and find bugs to work on at
[GitHub](https://github.com/iankronquist/rustyvisor/issues).
