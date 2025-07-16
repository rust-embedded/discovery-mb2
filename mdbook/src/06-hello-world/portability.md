# Portability

(This section is optional. Feel free to skip to the [next section], where we clean our code up a bit
and call it a day.)

[next section]: board-support-crate.html

You may wonder whether all this fancy ecosystem is worth its weight. The setup for our blinky is
pretty fancy, and uses a lot of Rust crates and features for such a simple job.

One cool advantage, though, is that our code becomes really portable. On a different board, the
setup may be different, but the actual blinky loop is identical!

Let's take a look at a blinky for the Sipeed Longan Nano. This is a little $5 board that, like the
MB2, is an embedded board with an MCU. Otherwise, it is completely different: different processor
(the GD32VF103, with a RISC-V instruction set entirely unlike the Arm instruction set we're using),
different peripherals, different board. But it has an LED attached to a GPIO pin, so we can blinky
it.

```rust
{{#include nanoblinky.rs}}
```

The differences in setup here are partly because different hardware, and partly because this code
uses an older HAL crate that hasn't yet been updated for `embedded-hal` 1.0. Yet the main loop is
identical as advertised, and the rest of the code is pretty recognizable. Because of the portability
provided by Rust's easy cross-compilation and the embedded Rust ecosystem, blinky is just blinky.

You can find a complete working [nanoblinky] example on GitHub, if you want to see all the
details or even get your own board and try it yourself.

[nanoblinky]: https://github.com/pdx-cs-rust/nanoblinky
