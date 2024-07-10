# Using a driver

As we already discussed in chapter 5 `embedded-hal` provides abstractions
which can be used to write platform independent code that can interact with
hardware. In fact all the methods we have used to interact with hardware
in chapter 7 and up until now in chapter 8 were from traits, defined by `embedded-hal`.
Now we'll make actual use of the traits `embedded-hal` provides for the first time.

It would be pointless to implement a driver for our LSM303AGR for every platform
embedded Rust supports (and new ones that might eventually pop up). To avoid this a driver
can be written that consumes generic types that implement `embedded-hal` traits in order to provide
a platform agnostic version of a driver. Luckily for us this has already been done in the
[`lsm303agr`] crate. Hence reading the actual accelerometer and magnetometer values will now
be basically a plug and play experience (plus reading a bit of documentation). In fact the `crates.io`
page already provides us with everything we need to know in order to read accelerometer data but using a Raspberry Pi. We'll
just have to adapt it to our chip:

[`lsm303agr`]: https://crates.io/crates/lsm303agr

Take a look at the linked page for the Raspberry Pi Linux sample code.

Because we already know how to create an instance of an object that implements the
[`embedded_hal::blocking::i2c`] traits from the [previous page](read-a-single-register.md), adapting
the sample code is straightforward (`examples/show-accel.rs`):

[`embedded_hal::blocking::i2c`]: https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/i2c/index.html

```rust
{{#include examples/show-accel.rs}}
```

Just like the last snippet you should just be able to try this out like this:
```console
$ cargo embed --example show-accel
```

Furthermore if you (physically) move around your micro:bit a little you should see the
acceleration numbers that are being printed change.
