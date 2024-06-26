# Naive approach and `write!`

## Naive approach

You probably came up with a program similar to the following:

```rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    for byte in b"The quick brown fox jumps over the lazy dog.\r\n".iter() {
        nb::block!(serial.write(*byte)).unwrap();
    }
    nb::block!(serial.flush()).unwrap();

    loop {}
}
```

While this is a perfectly valid implementation, at some point
you might want to have all the nice perks of `print!` such
as argument formatting and so on. If you are wondering how to do that, read on.

## `write!` and `core::fmt::Write`
The `core::fmt::Write` trait allows us to use any struct that implements
it in basically the same way as we use `print!` in the `std` world.
In this case, the `Uart` struct from the `nrf` HAL does implement `core::fmt::Write`
so we can refactor our previous program into this:

```rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use core::fmt::Write;

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();
    nb::block!(serial.flush()).unwrap();

    loop {}
}
```

If you were to flash this program onto your micro:bit, you'll
see that it is functionally equivalent to the iterator-based
program you came up with.
