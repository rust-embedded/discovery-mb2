# Inputs and Polling

In earlier chapters, we’ve explored GPIO pins primarily as outputs—driving LEDs on and off. However, GPIO pins can also be configured as inputs, allowing your program to read signals from the physical world, like button presses or switch toggles. In this chapter, we'll learn how to read these input signals and do something useful with them.

## Reading Button State

The micro:bit v2 has two physical buttons, Button A and Button B, connected to GPIO pins configured as inputs. Specifically, Button A is connected to pin P0.14, and Button B to pin P0.23. (You can verify this from the official [pinmap table].)

[pinmap table]: https://tech.microbit.org/hardware/schematic/#v2-pinmap

Reading the state of a GPIO input involves checking whether the voltage level at the pin is high (1) or low (0). The buttons on the micro:bit are connected to pins; when the buttons are pressed, they pull the voltage at the pin low (to 0V ground).

Let's now apply this knowledge to reading the state of Button A by checking if the button is "low" (pressed).

```rust
{{#include examples/button-a-bsp.rs}}
```
