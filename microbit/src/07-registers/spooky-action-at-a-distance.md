# Spooky action at a distance

`OUT` is not the only register that can control the pins of Port E. The `OUTSET` register also lets
you change the value of the pins, as can `OUTCLR`. However, `ODRSET` and `OUTCLR` don't let you retrieve the current output status of Port E.

`OUTSET` is documented in:

> Subsection 6.8.2.2. OUTSET - Page 145

Let's look at below program. The key to this program
is `fn print_out`. This function prints the current
value in `OUT` to the `RTT` console:

``` rust
#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, rprintln};

// Print the current contents of P0.OUT
fn print_out() {
    const P0_OUT: u32 = 0x5000_0504;

    let out = unsafe { ptr::read_volatile(P0_OUT as *const u32) };

    rprintln!("P0.OUT = {:#08x}", out);
}

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A bunch of magic addresses!
        const P0_OUTSET: u32 = 0x5000_0508;
        const P0_OUTCLR: u32 = 0x5000_050C;

        // Print the initial contents of OUT
        print_out();

        // Turn on the top LED row
        ptr::write_volatile(P0_OUTSET as *mut u32, 1 << 21);
        print_out();

        // Turn on the bottom LED row
        ptr::write_volatile(P0_OUTSET as *mut u32, 1 << 19);
        print_out();

        // Turn off the top LED row
        ptr::write_volatile(P0_OUTCLR as *mut u32, 1 << 21);
        print_out();

        // Turn off the bottom LED row
        ptr::write_volatile(P0_OUTCLR as *mut u32, 1 << 19);
        print_out();
    }

    loop {}
}
```

You'll see this if you run this program:

``` console
$ cargo embed
# cargo-embed's console
(..)
15:13:24.055: P0.OUT = 0x000000
15:13:24.055: P0.OUT = 0x200000
15:13:24.055: P0.OUT = 0x280000
15:13:24.055: P0.OUT = 0x080000
15:13:24.055: P0.OUT = 0x000000
```

Side effects! Although we are reading the same address multiple times without actually modifying it,
we still see its value change every time `OUTSET` or `OUTCLR` is written to.
