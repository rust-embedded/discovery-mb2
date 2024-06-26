# Hardware/knowledge requirements

The primary knowledge requirement to read this book is to know *some* Rust. It's
hard for me to quantify *some* but at least I can tell you that you don't need
to fully grok generics, but you do need to know how to *use* closures. You also
need to be familiar with the idioms of the current Rust [edition].

[edition]: https://rust-lang-nursery.github.io/edition-guide/

Also, to follow this material you'll need the following hardware:

- A [Micro:Bit v2] (MB2) board.

[micro:bit v2]: https://tech.microbit.org/hardware/

You can purchase this board from many suppliers, including
Amazon and Ali Baba. You can get a [list][0] of suppliers
directly from the BBC, the manufacturers of MB2.

[0]: https://microbit.org/buy/

<p align="center">
<img title="micro:bit" src="../assets/microbit-v2.jpg">
</p>

- One micro-B USB cable (nothing special — you probably have many of these). This is required
  to power the micro:bit board when not on battery, and to communicate with it.  Make sure
  that the cable supports data transfer, as some cables only support charging devices.

<p align="center">
<img title="micro-B USB cable" src="../assets/usb-cable.jpg">
</p>

> **NOTE** Some micro:bit kits ship with such cables.  USB cables used with other mobile
> devices should work, if they are micro-B and have the capability to transmit data.

> **FAQ**: Wait, why do I need this specific hardware?

It makes my life and yours much easier.

The material is much, much more approachable if we don't have to worry about hardware differences.
Trust me on this one.

> **FAQ**: Can I follow this material with a different development board?

Maybe? It depends mainly on two things: your previous experience with microcontrollers and/or
whether a high level crate already exists, like the [`nrf52-hal`], for your development board
somewhere. You can look through the [Awesome Embedded Rust HAL list] for your microcontroller,
if you intend to use a different one.

[`nrf52-hal`]: https://docs.rs/nrf52-hal
[Awesome Embedded Rust HAL list]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

With a different development board, this text would lose most if not all its beginner friendliness
and "easy to follow"-ness, in my opinion.

If you have a different development board and you don't consider yourself a total beginner, you are
better off starting with the [quickstart] project template.

[quickstart]: https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/
