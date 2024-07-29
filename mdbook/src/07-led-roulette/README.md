# LED roulette

Alright, let's start by building the following application:

<p align="center">
<video src="../assets/roulette_fast.mp4" width="500" loop autoplay/>
</p>

I'm going to give you a high level API to implement this app. Don't worry — we'll do low level stuff
later on. The main goal of this chapter is to get familiar with the *flashing* and debugging
process. "Flashing" is the process of writing a compiled program's code into "flash" memory on a
device: it has nothing to do with the LEDs.

The starter code is in the `src` directory of the book repository. Inside that directory there are
more directories named after each chapter of this book. Most of those directories are starter Cargo
projects.

Now, jump into the `src/07-led-roulette` directory. Check the `examples/init.rs` file:

``` rust
{{#include examples/init.rs}}
```

Microcontroller programs are different from standard programs in two aspects: `#![no_std]` and
`#![no_main]`.

The `no_std` attribute says that this program won't use the `std` crate, which assumes an underlying
OS; the program will instead use the `core` crate, a subset of `std` that can run on bare metal
systems (that is, systems without OS abstractions like files and sockets).

The `no_main` attribute says that this program won't use the standard `main` interface, which is
tailored for command line applications that receive arguments. Instead of the standard `main` we'll
use the `entry` attribute from the [`cortex-m-rt`] crate to define a custom entry point. In this
program we have named the entry point `main`, but any other name could have been used. The entry
point function must have signature `fn() -> !`; this type indicates that the function can't return.
This means that the program never terminates by returning from `main`: if the compiler detects that
this would be possible it will refuse to compile your program.

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

If you are a careful observer, you'll also notice there is a possibly-hidden `.cargo` directory in
the Cargo project as well. This directory contains a Cargo configuration file `.cargo/config.toml`.

```toml
{{#include .cargo/config.toml}}
```

This file tweaks the linking process to tailor the memory layout of the program to the requirements
of the target device.  This modified linking process is a requirement of the `cortex-m-rt`
crate. The `.cargo/config.toml` file also tells Cargo how to build and run code on our MB2.

There is also an `Embed.toml` file here:

```toml
{{#include Embed.toml}}
```

This file tells `cargo-embed` that:

- We are working with an NRF52833.
- We want to halt the chip after flashing it, so our program stops before `main`.
- We want to disable RTT. RTT is a protocol that allows the chip to send text to a debugger.
  You have already seen RTT in action: it was the protocol that sent "Hello World" in chapter 3.
- We want to enable GDB. This will be required for the debugging procedure.

Now that we've seen what's going on, let's start by building this program.
