# Verify the installation

Let's verify that all the tools were installed correctly.

## Verifying cargo-embed

First, connect the micro:bit to your Computer using a USB cable.

At least an orange LED right next to the USB port of the micro:bit should light up.  Furthermore, if
you have never flashed another program on to your micro:bit, the default program the micro:bit ships
with should start blinking the red LEDs on its back: you can ignore them, or you can play with the
demo app.

Now let's see if probe-rs, and by extensions cargo-embed can see your micro:bit. You can do this by
running the following command:

``` console
$ probe-rs list
The following debug probes were found:
[0]: BBC micro:bit CMSIS-DAP -- 0d28:0204:990636020005282030f57fa14252d446000000006e052820 (CMSIS-DAP)
```

Or if you want more information about the micro:bits debug capabilities then you can run:

``` console
$ probe-rs info
Probing target via JTAG

Error identifying target using protocol JTAG: The probe does not support the JTAG protocol.

Probing target via SWD

Arm Chip with debug port Default:
Debug Port: DPv1, DP Designer: Arm Ltd
├── 0 MemoryAP
│   └── ROM Table (Class 1), Designer: Nordic VLSI ASA
│       ├── Cortex-M4 SCS   (Generic IP component)
│       │   └── CPUID
│       │       ├── IMPLEMENTER: Arm Ltd
│       │       ├── VARIANT: 0
│       │       ├── PARTNO: Cortex-M4
│       │       └── REVISION: 1
│       ├── Cortex-M3 DWT   (Generic IP component)
│       ├── Cortex-M3 FBP   (Generic IP component)
│       ├── Cortex-M3 ITM   (Generic IP component)
│       ├── Cortex-M4 TPIU  (Coresight Component)
│       └── Cortex-M4 ETM   (Coresight Component)
└── 1 Unknown AP (Designer: Nordic VLSI ASA, Class: Undefined, Type: 0x0, Variant: 0x0, Revision: 0x0)


Debugging RISC-V targets over SWD is not supported. For these targets, JTAG is the only supported protocol. RISC-V specific information cannot be printed.
Debugging Xtensa targets over SWD is not supported. For these targets, JTAG is the only supported protocol. Xtensa specific information cannot be printed.
```

Next, make sure you are in `src/03-setup` of this book's source code. Then run these commands:

```
$ rustup target add thumbv7em-none-eabihf
$ cargo embed --target thumbv7em-none-eabihf
```

If everything works correctly cargo-embed should first compile the small example program
in this directory, then flash it and finally open a nice text based user interface that
prints Hello World.

(If it does not, check out [general troubleshooting] instructions.)

[general troubleshooting]: ../appendix/1-general-troubleshooting/index.html

This output is coming from the small Rust program you just flashed on to your micro:bit.
Everything is working properly and you can continue with the next chapters!
