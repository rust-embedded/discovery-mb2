# Naive approach and `write!`

## Naive approach

You probably came up with a program similar to the following (`examples/naive-send-string.rs`):

```rs
{{#include examples/naive-send-string.rs}}
```

While this is a perfectly valid implementation, at some point you might want to have all the nice
perks of `print!` such as argument formatting and so on. If you are wondering how to do that, read
on.

## `write!` and `core::fmt::Write`

The `core::fmt::Write` trait allows us to use any struct that implements it in basically the same
way as we use `print!` in the `std` world.  In this case, the `Uart` struct from the `nrf` HAL does
implement `core::fmt::Write` so we can refactor our previous program into this
(`examples/send-string.rs`):

```rs
{{#include examples/send-string.rs}}
```

If you flash this program onto your micro:bit, you'll see that it is functionally equivalent to the
iterator-based program you came up with.
