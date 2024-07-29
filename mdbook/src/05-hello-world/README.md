# Hello World

In the last section, you wrote a sort of "Hello World" program. But for embedded programmers, the
"real Hello World" is to blink an LED — any LED — on and off once per second. A program that does
this is commonly known as a "blinky".

Why blinky? Because this shows that you have enough control of the board you're working with to
perform this simple task. You can get a program loaded onto the machine and running, you can find
and turn on the appropriate pin on the MCU, you can delay for a fixed amount of time. Once you have
this much control, other tasks become much more straightforward.

In previous chapters, you found out several ways to load a program onto your MB2. Now it's just a
question of which pin you turn on and off, and how you delay between these actions.

Let's start by finding out how to work with the needed pins. There's a path you can follow for this
if you know how to read electronic circuit "schematic" diagrams. You can find the [MB2 schematic],
find an LED on that schematic that you want to turn on and off, and find what GPIO pins on the
nRF52833 are attached to that LED. (The MB2 is a bit unusual in this regard: usually an LED is
attached to just one pin that turns it on or off. The LED "display" on the MB2 is hooked up in a
more complicated way to allow turning on and off combinations of LEDs at once: a feature that we
will be using shortly.)

[MB2 schematic]: https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF

We will work with the LED in the upper-left corner of the MB2 display. Tracing the `ROW1` and `COL1`
wires this LED is connected to, we can see that they go to pins on the nRF52833 labeled
`AC17`/`P0.21` and `B11`/`AIN4`/`P0.28`. Digging further through the documentation we find that
`AC17` and `B11` are the row and column indices of the physical pins (solder balls, really) on the
bottom of the chip — useless to us. `AIN4` just means that this pin can act as an "Analog Input",
which is also currently useless to us. (It will come into play later.)

This leaves `P0.21` and `P0.28`. These labels correspond to bits in the memory of the nRF52833 that
can be turned on and off to get the LED to light up. Because electronics reasons, if pin `P0.21` is
turned on (thus outputting 3.3V) and pin `P0.28` is turned off (thus accepting voltage) the LED will
light up.

But what do we do in software to cause this to occur? We will work at the level of the
`nrf52833-hal` crate. The Hardware Abstraction Layer (HAL) is a chunk of software designed to make a
particular microcontroller easier to work with. As can be seen from the name, we have one for the
microcontroller on the MB2. It happens to contain everything needed to turn our target LED on.

Take a look at `examples/light-up.rs` in this chapter's directory, and then try running it.
You could use something fancy like before, but we have it set up so that

```
cargo run --example light-up
```

will load and run your program. That one LED should now be brightly lit!

``` rust
{{#include examples/light-up.rs}}
```

Note that we access the Peripheral Access Crate (PAC) for this chip through our HAL crate. There's a
complicated dance needed to get access to our pins. Finally, since we can just initialize the pins
to the right levels, we don't need to set them. Wiggling the pins is a topic for the next section.
