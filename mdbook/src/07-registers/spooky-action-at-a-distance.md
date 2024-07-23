# Spooky action at a distance

`OUT` is not the only register that can control the pins of Port E. The `OUTSET` register also lets
you change the value of the pins, as can `OUTCLR`. However, `ODRSET` and `OUTCLR` don't let you retrieve the current output status of Port E.

`OUTSET` is documented in:

> Subsection 6.8.2.2. OUTSET - Page 145

Let's look at below program. The key to this program
is `fn print_out`. This function prints the current
value in `OUT` to the `RTT` console:

``` rust
{{#include src/bin/spooky.rs}}
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
