# Waiting for an interrupt

You may have wondered why we have been using `asm::wfi()` (wait for instruction) in our main loop instead of something like `asm::nop()`.

As discussed before, `asm::nop()` means no-op(eration), and is an instruction that the CPU executes without doing anything .  We definitely could have used `asm::nop()` in our main loop instead, and the program would have behaved the same way.  The microcontroller, on the other hand, would behave differently. 

Calling `asm::wfi()` puts the CPU into "Wait For Interrupt" (WFI) mode.  When the CPU is in WFI mode, it will sleep until an interrupt wakes it up.  During sleep, the CPU will stop fetching instructions, turn off clocks and some peripherals, and enter a low-power state, but still keep the core running.  When an interrupt occurs, the CPU will wake up and execute as normal.

<br/>

The main difference between `asm::wfi()` and `asm::nop()` is that the NOP instruction is still an instruction.  It still needs to be fetched from the program memory and be executed even though the execution doesn't do anything.  Most microcontrollers you'll find out there have a low-power mode (some even have several, each with varying things staying on and each with different power consumption characteristics) that can, and *should* in a lot of cases, be used to save power.

You'll find some interrupt-driven programs that consist of nothing but `asm::wfi()` in the main loop, with all program logic being implemented in the interrupt handlers.
