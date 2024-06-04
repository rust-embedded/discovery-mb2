# RTRM: Reading The Reference Manual

I mentioned that the microcontroller has several pins. For convenience, these pins are grouped in
*ports*. There are two ports, Port 0 and Port 1, abbreviated to `P0` and `P1` respectively. The pins within each
port are named with numbers starting from 0. Port 0 has 32 pins, named `P0.00` to `P0.31`, and Port 1 has 10 pins, named `P1.00` to `P1.09`.

The first thing we have to find out is which pin is connected to which LED. This information is in
the microbit [hardware documentation]. Specifically, it's in the [pinmap table].

[hardware documentation]: https://tech.microbit.org/hardware/schematic/#v2-pinmap
[pinmap table]: https://tech.microbit.org/hardware/schematic/#v2-pinmap

The table says:

- `ROW1`, the top LED row, is connected to the pin `P0.21`. `P0.21` is the short form of: Pin 21 on Port 0.
- `ROW5`, the bottom LED row, is connected to the pin `P0.19`.

Up to this point, we know that we want to change the state of the pins `P0.21` and `P0.19` to turn the
top and bottom rows on and off. These pins are part of Port 0 so we'll have to deal with the `P0`
peripheral.

Each peripheral has a register *block* associated to it. A register block is a collection of
registers allocated in contiguous memory. The address at which the register block starts is known as
its base address. We need to figure out what's the base address of the `P0` peripheral. That
information is in the following section of the microcontroller [Product Specification]:

[Product Specification]: https://docs.nordicsemi.com/bundle/nRF52833_PS_v1.6/resource/nRF52833_PS_v1.6.pdf

> Section 4.2.4 Instantiation - Page 22

The table says that base address of the `P0` register block is `0x5000_0000`.

Each peripheral also has its own section in the documentation. Each of these sections ends with a
table of the registers that the peripheral's register block contains. For the `GPIO` family of
peripheral, that table is in:

> Section 6.8.2 Registers - Page 144

`OUT` is the register which we will be using to set/reset. Its offset value is `0x504` from the base address 
of the `P0`. We can look up `OUT` in the reference manual. 

That register is specified right under the `GPIO` registers table:

> Subsection 6.8.2.1 OUT - Page 145

Anyway, `0x5000_0000` + `0x504` = `0x50000504`. That looks familiar! Finally! 

This is the register we were writing to. The documentation says some interesting things. First, this
register can both be written to and read from. Next, the register is a 32-bit piece of memory, and each bit
represents the state of the corresponding pin. That means that bit 19 matches pin 19, for instance.
Setting the bit to 1 will enable the pin output, and setting it to 0 will reset it. Furthermore,
we can see that all pin outputs are disable by default, as the reset value of all bits is 0.

We'll use GDB's `examine` command: `x`. Depending on the configuration of your GDB server,
GDB will refuse to read memory that isn't specified. You can disable this behaviour by running:

```
set mem inaccessible-by-default off
```

So here we go. Furst turn off the `inaccessible-by-default` flag, then set a couple of breakpoints, reset the device and halt.

```
(gdb) set mem inaccessible-by-default off
(gdb) break 16
Breakpoint 1 at 0x172: file src/07-registers/src/main.rs, line 16.
Note: automatically using hardware breakpoints for read-only addresses.
(gdb) break 19
Breakpoint 2 at 0x17c: file src/07-registers/src/main.rs, line 19.
(gdb) break 22
Breakpoint 3 at 0x184: file src/07-registers/src/main.rs, line 22.
(gdb) break 25
Breakpoint 4 at 0x18c: file src/07-registers/src/main.rs, line 25.
(gdb) monitor reset halt
Resetting and halting target
Target halted
```

All right. Let's continue until the first breakpoint, right before line 16, and print the contents of the register at address `0x50000504`.

```
(gdb) c
Continuing.

Breakpoint 1, registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:16
16              *(PORT_P0_OUT as *mut u32) |= 1 << 21;
(gdb) x 0x50000504
0x50000504:     0x00000000
```

Ok, we see that the register's value is `0x00000000` or `0` at this point. This corresponds with the data in the product specification, which says that `0` is the 'reset value' of this register. That means that once the MCU resets, the register will have `0` as its value.

Let's go on. This line consists of multiple instructions (reading, bitwise ORing and writing), so we need to instruct the debugger to continue execution more than once, until we hit the next breakpoint.

```
(gdb) c
Continuing.

Program received signal SIGINT, Interrupt.
0x00000174 in registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:16
16              *(PORT_P0_OUT as *mut u32) |= 1 << 21;
(gdb) c
Continuing.

Breakpoint 2, registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:19
19              *(PORT_P0_OUT as *mut u32) |= 1 << 19;
```

We've stopped right before line 19, meaning that line 16 is fully executed at this point. Let's have a look at the `OUT` register's contents again:

```
(gdb) x 0x50000504
0x50000504:     0x00200000
```

The value of the `OUT` register is `0x00200000` at this point, which is `2097152` in decimal, or `2^21`. That means that bit 21 is set to 1, and the rest of the bits is set to 0. That corresponds to the code on line 16, which writes `1 << 21`, or a 1 shifted left 21 positions, bitwise ORed with `OUT`s current value (which was 0), to the `OUT` register.

Writing `1 << 21` (`OUT[21]= 1`)  to `OUT`  sets `P0.21` *high*. That turns the top LED row *on*. Check that the top row is now indeed lit up.

```
(gdb) c
Continuing.
```

Yeah, I was gonna say that. Now, hit 'c' another time to continue execution up to the next breakpoint and print its value.

```
Program received signal SIGINT, Interrupt.
0x0000017e in registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:19
19              *(PORT_P0_OUT as *mut u32) |= 1 << 19;
(gdb) c
Continuing.

Breakpoint 3, registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:22
22              *(PORT_P0_OUT as *mut u32) &= !(1 << 21);
(gdb) x 0x50000504
0x50000504:     0x00280000
```

On line 19, we've set bit 21 of `OUT` to 1, keeping bit 19 as is. The result is `0x00280000`, wich is `2621440` in decimal, or `2^19 + 2^21`, meaning that both bit 19 and bit 21 is set to 1.

Writing `1 << 19` (`OUT[19]= 1`) to `OUT` sets `P0.19` *high*. That turns the bottom LED row *on*. As such, the bottom row should now be lit up.

The following lines turn the rows off again. First the top row, then the bottom row. This time, we're doing a bitwise AND operation, combined with a bitwise NOT. We calculate  `!(1 << 21)`, which is all bits set to 1, except for bit 21. Next, we bitwise AND that with the current value of `OUT`, ensuring that only bit 21 is set to 0, keeping the value of the other bits intact.

Continue execution and check that the reported values of the `OUT` register matches what you expect. You can press `CTRL+C` to pause execution once the device enters the endless loop at the end of the `main` function.

```
(gdb) c
Continuing.

Program received signal SIGINT, Interrupt.
0x00000186 in registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:22
22              *(PORT_P0_OUT as *mut u32) &= !(1 << 21);
(gdb) c
Continuing.

Breakpoint 4, registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:25
25              *(PORT_P0_OUT as *mut u32) &= !(1 << 19);
(gdb) x 0x50000504
0x50000504:     0x00080000
(gdb) c
Continuing.

Program received signal SIGINT, Interrupt.
0x0000018e in registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:25
25              *(PORT_P0_OUT as *mut u32) &= !(1 << 19);
(gdb) c
Continuing.
^C
Program received signal SIGINT, Interrupt.
0x00000196 in registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:28
28          loop {}
(gdb) x 0x50000504
0x50000504:     0x00000000
```

And at this points all LEDs should be turned off again!