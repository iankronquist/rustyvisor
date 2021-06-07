KDIR := /home/ian/linux

obj-m := rustyvisor.o
rustyvisor-y := src/linux_module.o  src/isr.o src/host_entrypoint.o liblinux.o
PWD := $(shell pwd)


RUST_RELEASE := debug
RUST_TARGET := x86_64-unknown-none-linuxkernel
RUST_LIB_NAME := linux
RUST_LIB_PATH := ../target/$(RUST_TARGET)/$(RUST_RELEASE)/lib$(RUST_LIB_NAME).a
RUST_FILES := src/*.rs
CARGO := cargo

all: liblinux.o
	$(MAKE) -C $(KDIR) M=$(PWD) modules

$(RUST_LIB_PATH): $(RUST_FILES) Cargo.toml
	$(CARGO) build --target=$(RUST_TARGET)

liblinux.o: $(RUST_LIB_PATH)
	cp $< $@
	echo "cmd_$(realpath $@) := cp $< $@" > .liblinux.o.cmd

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean

clean-all: clean
	$(CARGO) clean
