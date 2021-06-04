# Building as a Linux Kernel Module

These instructions are deprecated but are kept around in case I want to make
the hypervisor run as a Linux kernel module again.

### Launching Vagrant

A Vagrantfile is included to aid development and testing. It requires libvirt,
QEMU, and the vagrant-libvirt plugin. Additionally, your system must support NFS
and nested KVM.

To launch the Vagrant box, simply run `vagrant up` in the repo root and cross
your fingers. Once launched, access the VM with `vagrant ssh`, and cd to
/vagrant. You can then build and launch Rustyvisor as described below.

### Installing dependencies manually

RustyVisor depends on nightly Rust, Xargo, GCC, and the Linux kernel module
development headers.

To install the Linux kernel headers and GCC on Ubuntu:
```
$ sudo apt-get install linux-headers-$(uname -r) gcc
```

On Fedora you can use:
```
$ sudo dnf install kernel-devel-$(uname -r) gcc
```

And for Arch Linux:
```
$ sudo pacman -S linux-headers gcc
```

Next install the nightly rust toolchain:
```
$ # Bootstrap rustup to get a nightly build of Rust.
$ curl https://sh.rustup.rs -sSf | sh
$ source ~/.cargo/env
$ rustup install nightly
$ rustup default nightly
```

To build Rust code for kernel context we use the cross compilation tool Xargo:
```
$ rustup component add rust-src
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

