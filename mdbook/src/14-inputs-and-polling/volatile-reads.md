# Volatile Reads

Reading from registers introduces a subtlety: compiler optimization. Consider this snippet, reading directly from a GPIO register:

```rust
use core::ptr;

// Direct memory access to GPIO IN register 
const GPIO_P0_IN: u32 = 0x50000510;

// NOT RECOMMENDED (optimization may break correctness)
fn button_pressed() -> bool {
    unsafe {
        *(GPIO_P0_IN as *const u32) & (1 << 14) != 0
    }
}

#[entry]
fn main() -> ! {
    let (p0, _p1) = registers::init();

    loop {
        let button_a_pressed = button_pressed();
        if button_a_pressed {
            rprintln!("Button A pressed");
        } else {
            rprintln!("Button A not pressed");
        }
    }
}
```

Here we have code that looks like it should work as intended, i.e. it repeatedly reads the state of Button A and prints the result. Unfortunately, as discussed in the "(mis)Optimization" section of Chapter 7, reading or writing directly to memory-mapped registers through address dereferencing is likely to produce incorrect results.  The compiler might mistakenly assume these registers are regular memory locations and cache the reads or writes in registers, only reading the value from the register once and using that cached value for all subsequent reads, regardless of the actual state of the register.

As you may have guessed, we need to do *volatile* reads instead.  Here is a better implementation of the `button_pressed` function:

```rust
fn button_a_pressed() -> bool {
    unsafe {
        let reg = ptr::read_volatile(GPIO_P0_IN as *const u32);
        (reg & (1 << 14)) == 0 // Active-low logic
    }
}
```

This code performs a *volatile* read from the GPIO IN register, ensuring that every access directly targets the memory-mapped register address and doesn't get optimized away.  

Thankfully, the micro:bit's Board Support Crate (BSP) abstracts away these low-level volatile reads entirely, allowing us to read button states in a simpler way:

```rust
{{#include examples/button-a-bsp.rs}}
```

It's nice when you can work with a higher-level abstraction, and the micro:bit BSP makes this easy.  You won't always be so lucky.  Hopefully, now you know how to interact with registers directly, and you should be equipped to implement your own higher-level abstraction when none is available.