# RustyVisor  [![build_status](https://travis-ci.com/iankronquist/rustyvisor.svg?branch=master)](https://travis-ci.com/iankronquist/rustyvisor)

A hypervisor written in Rust.

The goal of this project is to learn more Rust and determine whether Rust is
a significant improvement over C for systems programming tasks like
implementing hypervisors.

This project takes the form of a uefi runtime service which virtualizes the
uefi environment. After running the driver UEFI will be running inside a VM as
a guest and the host operating system will be RustyVisor!

There is also some work in progress code in the linux/ directory which builds
the hypervisor as a Linux kernel module.

This code is relatively exploratory, and a work in progress, so please excuse
the state of the code and this rough excuse for documentation.


## Building the Hypervisor as a UEFI Runtime Service

To build the hypervisor as a uefi application, you will need a nightly rust and
a version of clang which supports cross compiling with the
`x86_64-unknown-windows` target and the tool llvm-lib, included with LLVM. The
clang included with most Linux distributions should work. OS X users may have
to install a version of clang from homebrew, as I don't believe that the
version which ships with xcode doesn't support cross compiling PE binaries.
Windows users are encouraged to use the Windows Subsystem for Linux, possibly
with an Ubuntu distribution.

First, install the necessary rust dependencies with rustup:
```
rustup install nightly
rustup default nightly
rustup target install x86_64-unknown-uefi
```

Once you have the right version of rust installed, building is straightforward:

```
sh build.sh
```

## Launching from a UEFI shell

First, build the project, and copy the hypervisor from
`target/x86_64-unknown-uefi/debug/rustyvisor.efi` onto a USB stick. 


Unmount the USB stick from your development device and insert it into your test
device, assuming you have a separate test and development devices.
If you're running under the Windows subsystem for Linux, check out the
powershell script under `scripts/deploy_wsl.ps1` which automates some of the
mounting and copying.


Then, boot your test hardware to a UEFI shell. Some test hardware boots to a
UEFI shell by default, but most of the time it may be easiest to copy the UEFI
shell firmware onto the USB stick and boot from the USB stick. For now, the
instructions for booting into a UEFI shell are outside the scope of this
document.

At the UEFI shell, identify the UEFI filesystem mapping which represents the
USB. In this example, it's fs0.
Then, load the hypervisor with the UEFI shell load command:

```
UEFI Interactive Shell v2.2
EDK II
UEFI v2.70 (EDK II, 0x00010000)
Mapping table
      FS0: Alias(s):F0a:;BLK0:
          PciRoot(0x0)/Pci(0x1,0x1)/Ata(0x0)
Press ESC in 1 seconds to skip startup.nsh or any other key to continue.
Shell> fs0:
FS0:\> dir
Directory of: FS0:\
06/03/2021  23:33             342,016  rustyvisor.efi
06/03/2021  23:42              10,383  NvVars
          2 File(s)     433,807 bytes
          0 Dir(s)
FS0:\> load .\rustyvisor.efi
FS0:\>
```

## Running the UEFI Runtime Service Hypervisor Under the Bochs Emulator

To make development without real hardware easier, there is a bochs
configuration file under `uefi/bochsrc.txt`. This bochsrc is setup to boot to a
UEFI shell with a FAT filesystem mounted.

Unfortunately, you may have to adjust the bochs config file for your system. In
partiticular, you may want to update the display library and the paths to the
UEFI shell and the BIOS and VGA ROM images.


On ubuntu, to run the hypervisor from a UEFI shell under bochs, you will need
to install these packages:
```
sudo apt install bochs # The emulator we're using to test the hypervisor.
sudo apt install ovmf # The uefi shell binary
# The next two are used by scripts to build the filesystem image.
# If you want to build the filesystem another way, say by mounting on the
# loopback device, that's okay too.
sudo apt install dosfstools # Used to make the fat filesystem. 
sudo apt install mtools # Used to copy the hypervisor onto the fat filesystem.
```

Several scripts have been included to make this process easier.

* `scripts/make_uefi_test_fs.sh`
   will create a FAT filesystem image named fat.img for bochs to use.
* `scripts/bochs_deploy.sh` will copy the uefi hypervisor image onto the FAT
   filesystem created by the previous script.

Assuming you have installed the necessary dependencies, adjusted the bochs
configuration file to match your system, if necessary, and created the
filesystem image with the tools under scripts, starting bochs should be
relatively simple:
```
bochs -qf ./uefi/bochsrc.txt
...
com1: waiting for client to connect (host:localhost, port:14449)
```
Bochs will stop before begining emulation twice. First, it will wait for a
client to connect to its COM1 serial port emulator. I recommend running telnet
in another terminal:

```
$ telnet localhost 14449
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
```

Then, once you have connected to the emulated serial terminal, you will need to start emulation from the bochs debugger:
```
<bochs:1> continue
```

After a few seconds, bochs should start up the UEFI shell and you should see a prompt from the telnet session:
```
Shell>
```
From here you can follow the instructions under the section called Launching from a UEFI shell.

## Building the Hypervisor as a Linux Kernel Module

Note that this is a work in progress, and likely to crash your machine.

First, install the Linux kernel headers for your kernel:
```
sudo apt install linux-headers-$(uname -r)
```

Next, build the hypervisor using make. Do not invoke cargo directly, let make handle that.

```
$ cd linux/
$ make
make -C /home/ian/linux M=/home/ian/rustyvisor/linux modules
make[1]: Entering directory '/home/ian/linux'
  CC [M]  /home/ian/rustyvisor/linux/src/linux_module.o
  AS [M]  /home/ian/rustyvisor/linux/src/isr.o
/home/ian/rustyvisor/linux/src/isr.o: warning: objtool: .text+0x0: unreachable instruction
  AS [M]  /home/ian/rustyvisor/linux/src/host_entrypoint.o
/home/ian/rustyvisor/linux/src/host_entrypoint.o: warning: objtool: .text+0x0: unreachable instruction
  LD [M]  /home/ian/rustyvisor/linux/rustyvisor.o
  MODPOST /home/ian/rustyvisor/linux/Module.symvers
  CC [M]  /home/ian/rustyvisor/linux/rustyvisor.mod.o
  LD [M]  /home/ian/rustyvisor/linux/rustyvisor.ko
make[1]: Leaving directory '/home/ian/linux'
```
This should produce a file named rustyvisor.ko.

If your kernel has KVM installed, you may need to remove it with the script remove-kvm.sh:
```
$ sh ./scripts/remove-kvm.sh
```

Load the hypervisor with the insmod command:
```
$ sudo insmod rustyvisor.ko
```

## Contributions & Bugs

Pull requests welcome! This is a project in part to learn about Rust, so if
there is code which isn't idiomatic, please feel free to show me a better way.

Please file bugs and find bugs to work on at
[GitHub](https://github.com/iankronquist/rustyvisor/issues).
