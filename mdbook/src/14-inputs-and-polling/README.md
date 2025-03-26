# Inputs and Polling

In earlier chapters, we’ve explored GPIO pins primarily as outputs—driving LEDs on and off. However, GPIO pins can also be configured as inputs, allowing your program to read signals from the physical world, like button presses or switch toggles. In this chapter, we'll learn how to read these input signals and do something useful with them.

## GPIO Inputs

Recall from the Registers chapter that GPIO pins on the micro:bit are grouped into two ports (P0 and P1), each with its own register block. We've previously manipulated the OUT register to drive pins high or low, activating LEDs. Now, we'll explore another register, IN, which enables us to read the state of input pins.

### Reading Button State

The micro:bit v2 has two physical buttons, Button A and Button B, connected to GPIO pins configured as inputs. Specifically, Button A is connected to pin P0.14, and Button B to pin P0.23. (You can verify this from the official pinmap table.)

Reading the state of a GPIO input involves checking whether the voltage level at the pin is high (1) or low (0). 
In the Registers chapter, we learned how to manipulate GPIO registers directly using a type-safe API, but under the hood, the API is just a wrapper around the raw register values. When the voltage level at the pin is high, the corresponding bit in the IN register is set to 1. Let's now apply this knowledge to reading the state of Button A (connected to P0.14) by accessing the IN register, which reflects the current input state of GPIO pins:

```rust
{{#include examples/button-a.rs}}
```

In this snippet:

We access the type-safe API provided by the registers module, specifically reading from the IN register of port P0.

Using pin14().bit_is_clear() conveniently checks if pin 14 reads low (0), indicating Button A is pressed (active-low logic).