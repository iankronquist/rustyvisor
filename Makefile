TARGET := x86_64-linux
RELEASE := debug
MODULENAME := rustyvisor
obj-m += $(MODULENAME).o
$(MODULENAME)-objs += loader/linux.o target/$(TARGET)/$(RELEASE)/lib$(MODULENAME).a
KDIR := /lib/modules/$(shell uname -r)/build
PWD := $(shell pwd)
CARGO := cargo
XARGO := xargo
RUST_FILES := libs/*/src/*.rs src/*.rs

all: $(MODULENAME).ko

$(MODULENAME).ko: loader/linux.c target/$(TARGET)/$(RELEASE)/lib$(MODULENAME).a
	$(MAKE) -C $(KDIR) SUBDIRS=$(PWD) modules

target/$(TARGET)/$(RELEASE)/lib$(MODULENAME).a: $(RUST_FILES) Cargo.toml
	RUSTFLAGS='-C relocation-model=static' $(XARGO) build --target=$(TARGET) --verbose

test:
	cd libs/allocator && $(CARGO) test
	$(CARGO) test


clean:
	rm -f *.o *.ko *.ko.unsigned modules.order Module.symvers *.mod.c .*.cmd
	rm -rf .tmp_versions
	$(XARGO) clean

.PHONY: all clean test
