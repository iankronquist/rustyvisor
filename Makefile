RELEASE := debug
MODULENAME := rustyvisor
obj-m += $(MODULENAME).o
$(MODULENAME)-objs += loader/linux.o target/$(RELEASE)/lib$(MODULENAME).a
KDIR := /lib/modules/$(shell uname -r)/build
PWD := $(shell pwd)
SIGNFILE := /usr/src/linux-headers-$(shell uname -r)/scripts/sign-file
CARGO := cargo

all: $(MODULENAME).ko

$(MODULENAME).ko: loader/linux.c target/$(RELEASE)/lib$(MODULENAME).a
	$(MAKE) -C $(KDIR) SUBDIRS=$(PWD) modules

target/$(RELEASE)/librustyvisor.a: src/*.rs Cargo.toml
	RUSTFLAGS='-C relocation-model=static' $(CARGO) build

clean:
	rm -f *.o *.ko *.ko.unsigned modules.order Module.symvers *.mod.c .*.cmd
	rm -rf .tmp_versions
	$(CARGO) clean

.PHONY: all clean
