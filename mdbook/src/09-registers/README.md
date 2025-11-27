# Registers

This chapter is a technical deep-dive. You can safely [skip it] for now and come back to it later if
you like. That said, there's a lot of good stuff in here, so I'd recommend you dive in.

[skip it]: ../10-serial-port/index.html

-----

It's time to explore what calling `display_pins.row1.set_high()` or `button_a_pin.is_high()` does under the hood.

In a nutshell, calling `display_pins.row1.set_high()` just writes to some special memory regions. Go into the `09-registers` directory
and let's run the starter code statement by statement (`src/main.rs`).

``` rust
{{#include src/main.rs}}
```

What's this magic?

The address `0x50000504` points to a *register*. A register is a special region of memory that
controls a *peripheral*. A peripheral is a piece of electronics that sits right next to the
processor within the microcontroller package and provides the processor with extra functionality.
After all, the processor, on its own, can only do math and logic.

This particular register controls General Purpose Input/Output (GPIO) *pins* (GPIO *is* a
peripheral) and can be used to *drive* each of those pins
*low* or *high*. 

(On the nRF52833 there are more than 32
GPIOs, yet the CPU is 32-bit. Thus, the GPIO
pins are organized in two groups "P0" and "P1", with a set of registers
for reading, writing and configuring each group. The address
above is the address of the output register for the P0 pins.)

## An aside: LEDs, digital outputs and voltage levels

Drive? Pin? Low? High?

A pin is a electrical contact. Our microcontroller has several of them and some of them are
connected to Light Emitting Diodes (LEDs). An LED will emit light when voltage is applied to it.  As
the name implies, an LED also acts as a "diode". A diode will only let electricity flow in one
direction. Hook an LED up "forwards" and light comes out. Hook it up "backwards" and nothing
happens.

<p align="center">
<img class="white_bg" height="180" title="LED circuit" src="https://upload.wikimedia.org/wikipedia/commons/c/c9/LED_circuit.svg" />
</p>

Luckily for us, the microcontroller's pins are connected such that we can drive the LEDs the right
way round. All that we have to do is apply enough voltage across the pins to turn the LED on. The
pins attached to the LEDs are normally configured as *digital outputs* and can output two different
voltage levels: "low", 0 Volts, or "high", 3 Volts. A "high" (voltage) level will turn the LED on
whereas a "low" (voltage) level will turn it off.

These "low" and "high" states map directly to the concept of digital logic. "low" is `0` or `false`
and "high" is `1` or `true`. This is why this pin configuration is known as digital output.

The opposite of a digital output is a digital input.  In the same way that a digital output can be either `0` or `1`, a digital input can be either `0` or `1`.  The difference is that digital outputs can drive a voltages, but digital inputs *read* a voltage.  When the microcontroller reads a voltage level above a high threshold, it will interpret that as a `1` and when it reads a voltage level below a low threshold, it will interpret that as a `0`. 

-----

OK. But how can one find out what this register does? Time to RTRM (Read the Reference Manual)!
