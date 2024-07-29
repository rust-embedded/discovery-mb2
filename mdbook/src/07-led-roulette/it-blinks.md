# It blinks

The equivalent of "hello world" for embedded programming is known as "blinky": make an LED on the
board blink on and off. As with "hello world" in high-level programs, this verifies that you
understand the basic operations needed for a very simple task, and that a bunch of things are
working as expected.

## Delaying

Let's first take a brief look into delay abstractions provided by `embedded-hal`.  We will then
combine this with the GPIO abstractions from the previous chapter in order to finally make an LED
blink.

`embedded-hal` provides us with an abstraction to delay the
execution of our program: the [`DelayNs`] trait.

[`DelayNs`]: https://docs.rs/embedded-hal/1.0.0/embedded_hal/delay/trait.DelayMs.html

Our MCU contains many "timer" peripherals, and several other timer-like devices. As the name
implies, timers can do various things regarding time for us, including simply pausing the execution
of our program for a fixed amount of time. A very simple delay-based program that prints something
every second might for example look like this code, in `examples/delay-print.rs`:

```rs
{{#include examples/delay-print.rs}}
```

Note that we changed our panic implementation from `panic_halt` to `panic_rtt_target` here.

In order to actually see the prints we have to change `Embed.toml` to turn off `gdb` and
`halt_afterwards` and turn on `rtt`, like this:
```
[default.general]
chip = "nrf52833_xxAA"

[default.reset]
halt_afterwards = false

[default.rtt]
enabled = true

[default.gdb]
enabled = false
```

And now with a quick `cargo embed --example delay-print` you should see "`1000 ms passed`" being
sent to your console every second from your MCU.

## Blinking

Now we've arrived at the point where we can combine our new knowledge about GPIO and delay
abstractions in order to actually make an LED on the micro:bit blink. The resulting program is
really just a mash-up of the one above and the one that turned an LED on in the last section and
looks like this:

```rs
```

After`cargo embed --example blinky` (with the proper `Cargo.toml`) you should see the LED we light
up before blinking, as well as a print every time the LED changes from off to on and vice versa.

Note that a more convenient way to run this code, since we don't need any of the fancy features of
`cargo embed`, is just to say `cargo run --example blinky`. This will flash our blinky and start it
running. You can take a look at `.cargo/config.toml` for how `cargo run` is invoking `probe-rs run`
to flash and run our program.
