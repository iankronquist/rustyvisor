KDIR ?= /lib/modules/$(shell uname -r)/build

obj-m := rustyvisor.o
rustyvisor-y := src/linux_module.o  src/isr.o src/host_entrypoint.o liblinux.o
PWD := $(shell pwd)


RUST_RELEASE := debug
RUST_TARGET := x86_64-unknown-none-linuxkernel
RUST_LIB_NAME := linux
RUST_LIB_PATH := ../target/$(RUST_TARGET)/$(RUST_RELEASE)/lib$(RUST_LIB_NAME).a
RUST_FILES := src/*.rs
CARGO := cargo


all: rustyvisor.ko

-include $(RUST_LIB_PATH:.a=.d)

rustyvisor.ko: liblinux.o
	echo $(RUST_LIB_PATH)
	$(MAKE) -C $(KDIR) M=$(PWD) modules

$(RUST_LIB_PATH): $(RUST_FILES) Cargo.toml
	$(CARGO) build --target=$(RUST_TARGET)

liblinux.o: $(RUST_LIB_PATH)
	@cp $(realpath $<) $@
	@echo "cmd_$(realpath $@) := cp $< $@" > .liblinux.o.cmd

rust-only: $(RUST_LIB_PATH)

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean

clean-all: clean
	$(CARGO) clean

.PHONY: clean clean-all rust-only
