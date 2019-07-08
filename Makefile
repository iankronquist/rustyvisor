TARGET := x86_64-linux
RELEASE := debug
MODULENAME := rustyvisor
obj-m += $(MODULENAME).o
$(MODULENAME)-objs += loader/linux.o target/$(TARGET)/$(RELEASE)/lib$(MODULENAME).a
KDIR := /lib/modules/$(shell uname -r)/build
LDFLAGS += -T $(SUBDIRS)/loader/linux_linker.lds
CARGO := cargo
KCOV := kcov
RUSTFILES := src/*.rs
RUSTFLAGS := '-C relocation-model=static --deny warnings'
CARFOFEATURES := runtime_tests
ccflags-y := -Wall -Werror

all: $(MODULENAME).ko

$(MODULENAME).ko: loader/linux.c target/$(TARGET)/$(RELEASE)/lib$(MODULENAME).a
	$(MAKE) -C $(KDIR) SUBDIRS=$(PWD) modules

target/$(TARGET)/$(RELEASE)/lib$(MODULENAME).a: $(RUSTFILES) Cargo.toml
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) xbuild --target=$(TARGET).json --verbose --features "$(CARFOFEATURES)"

test:
	$(CARGO) test --verbose

coverage:
	$(CARGO) test --no-run
	$(KCOV) --exclude-pattern=/.cargo,/usr/lib --verify target/cov target/debug/$(MODULENAME)-*

clean:
	rm -f *.o *.ko *.ko.unsigned modules.order Module.symvers *.mod.c .*.cmd $($(MODULENAME)-objs)
	rm -rf .tmp_versions
	rm -f $($(MODULENAME)-objs)
	$(CARGO) clean

.PHONY: all clean coverage test
