# Board support crate

Working directly with the PAC and HAL is pretty neat. Most Arm MCUs and many other MCUs that Rust
can compile for have a PAC crate. If you are working with one that does not, writing a PAC crate can
be tedious but is pretty straightforward. Many MCUs that have a PAC crate also have a HAL crate —
again, it's mostly just tedious work to build one if it is absent. Code written at the PAC and HAL
level gives access to the fine details of the MCU.

As we have seen, though, it becomes pretty annoying to keep track of just what is going on at the
interface between our nRF52833 and the rest of our MB2. We have had to read schematics and whatnot
to see how to use our off-board hardware.

A "board support crate" — known in the non-Rust embedded community as a Board Support Package (BSP)
— is a crate built on top of the HAL and PAC for a board to abstract away the details and provide
conveniences. The board support crate we have been working with is the `microbit-v2` crate.

Let's use `microbit-v2` to get a final, cleaned up blinky (`src/main.rs`).

```rust
{{#include src/main.rs}}
```

In this case, we haven't changed much. Our board support crate has hidden the PAC (for now). More
importantly, it has done so by letting us just use reasonable names for the row and column GPIO pins
for the LED.

The `microbit-v2` crate provides even fancier support for those "display" LEDs. We will see this
support used soon to do things more fun than blinky.
