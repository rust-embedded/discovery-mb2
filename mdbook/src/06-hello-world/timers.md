# Timers

One of the big advantages of a "bare-metal" embedded system is that you control everything that
happens on your machine. This allows you to have really precise control of time: nothing will slow
you down unless you let it.

However, we've seen that if we really want to get time right, we probably need help. Embedded MCUs
like the nRF52833 all provide this kind of help in the form of "timers". A timer is a peripheral
that, as its name implies, acts like a little clock that keeps very precise track of time.

The nRF52833 contains four timers. If you look at the documentation for the chip, you'll find that
they are pretty complicated to set up and use. Luckily, the HAL provides a wrapper around timers
that makes common uses easy. The most common use of a timer is to delay for a precise amount of
time: just what our `wait()` function of the previous sections was trying to do.

Take a look at `examples/timer-blinky.rs`. This code sets up a timer and uses it to delay for 500ms
(0.5s) between each toggle.

```rust
{{#include examples/timer-blinky.rs}}
```

Run this code with `cargo run --release --example timer-blinky` and time it with a stopwatch. You'll
find that it is exactly one second for each on-off cycle.

Things you might notice:

* We need to use the `embedded_hal::Delay` trait to get the `delay_ms()` method we're using.

* As before, we dig the peripheral out of the PAC peripherals struct and give it to the HAL.

Now we have a production-quality blinky. Let's talk a bit about the implications of all this.
