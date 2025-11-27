# Flash it

Flashing is the process of moving our program into the microcontroller's persistent memory. Once
flashed, the microcontroller will execute the flashed program every time it is powered on.

Our program will be the *only* program in the microcontroller memory.  By this I mean that there's
nothing else running on the microcontroller: no OS, no "daemon", nothing. Our program has full
control over the device.

Flashing the binary itself is quite simple, thanks to `cargo embed`.

Before executing that command though, let's look into what it actually does. If you look at the side
of your micro:bit with the USB connector facing upwards, you will notice that there are actually
three black squares on there. The biggest one is a speaker. Another is our MCU we already talked
about… but what purpose does the remaining one serve? This chip is *another* MCU, an NRF52820 almost
as powerful as the NRF52833 we will be programming! This chip has three main purposes:

1. Enable power and reset control of our NRF52833 MCU from the USB connector.
2. Provide a [serial to USB bridge] for our MCU.
3. Provide an interface for programming and debugging our NRF52833 (this is the relevant purpose for
   now).

This chip acts as sort of bridge between our computer (to which it is connected via USB) and the MCU
(to which it is connected via traces and communicates with using the SWD protocol). This bridge
enables us to flash new binaries on to the MCU, inspect a program's state via a debugger and do
other useful things.

So lets flash it!

```console
$ cargo embed --example init
  (...)
     Erasing sectors ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  4.21KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  2.71KiB/s (eta 0s )
    Finished flashing in 0.608s
```

You will notice that `cargo-embed` does not exit after outputting the last line. This is intended:
you should not close `cargo-embed`, since we need it in this state for the next step — debugging it!
Furthermore, you will have noticed that `cargo build` and `cargo embed` are actually passed the same
flags. This is because `cargo embed` actually executes the build and then flashes the resulting
binary on to the chip. This means you can leave out the `cargo build` step in the future if you want
to flash your code right away.

[serial to USB bridge]: ../10-serial-port/index.html
