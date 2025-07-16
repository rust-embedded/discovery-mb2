# Linux

Here are the installation commands for a few Linux distributions.

## Ubuntu 20.04 or newer / Debian 10 or newer

> **NOTE** `gdb-multiarch` is the GDB command you'll use to debug your Arm Cortex-M programs.
``` console
$ sudo apt install gdb-multiarch minicom libunwind-dev
```

## Fedora 32 or newer

> **NOTE** `gdb` is the GDB command you'll use to debug your Arm
> Cortex-M programs.
``` console
$ sudo dnf install gdb minicom libunwind-devel
```

## Arch Linux

> **NOTE** `gdb` is the GDB command you'll use to debug your Arm
> Cortex-M programs.
``` console
$ sudo pacman -S arm-none-eabi-gdb minicom libunwind
```

## Other distros

> **NOTE** `arm-none-eabi-gdb` is the GDB command you'll use to debug your Arm Cortex-M programs.

For distros that don't have packages for [Arm's pre-built
toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads), download the "Linux
64-bit" file and put its `bin` directory on your path.  Here's one way to do it:

``` console
$ mkdir -p ~/local
$ cd ~/local
$ tar xjf /path/to/downloaded/XXX.tar.bz2
```

Then, use your editor of choice to append to your `PATH` in the appropriate shell init file
(e.g. `~/.zshrc` or `~/.bashrc`):

```
PATH=$PATH:$HOME/local/XXX/bin
```

## udev rules

These rules let you use USB devices like the micro:bit without root privilege, i.e. `sudo`.

Create this file in `/etc/udev/rules.d` with the content shown below.

``` console
$ cat /etc/udev/rules.d/69-microbit.rules
```

``` text
# CMSIS-DAP for microbit
ACTION!="add|change", GOTO="microbit_rules_end"
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", TAG+="uaccess"
LABEL="microbit_rules_end"
```

Then reload the udev rules with:

``` console
$ sudo udevadm control --reload
```

If you had any board plugged to your computer, unplug them and then plug them in again, or run the
following command.

``` console
$ sudo udevadm trigger
```

## Verify permissions

Connect the micro:bit to your computer using a USB cable.

The micro:bit should now appear as a USB device (file) in `/dev/bus/usb`. Let's find out how it got
enumerated:

``` console
$ lsusb | grep -i "NXP Arm mbed"
Bus 001 Device 065: ID 0d28:0204 NXP Arm mbed
$ # ^^^        ^^^
```

In my case, the micro:bit got connected to the bus #1 and got enumerated as the device #65. This means the
file `/dev/bus/usb/001/065` *is* the micro:bit. Let's check the file permissions:

``` console
$ ls -l /dev/bus/usb/001/065
crw-rw-r--+ 1 nobody nobody 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
```

The permissions should be `crw-rw-r--+`, note the `+` at the end, then see your access rights by running the following command.

``` console
$ getfacl /dev/bus/usb/001/065
getfacl: Removing leadin '/' from absolute path names
# file: dev/bus/usb/001/065
# owner: nobody
# group: nobody
user::rw-
user:<YOUR-USER-NAME>:rw-
group::rw-
mask::rw-
other::r-
```

You should see your username in the list above with the
`rw-` permissions, if not ... then check your [udev rules]
and try re-loading them with:

[udev rules]: linux.md#udev-rules

``` console
$ sudo udevadm control --reload
$ sudo udevadm trigger
```

Now, go to the [next section].

[next section]: verify.md
