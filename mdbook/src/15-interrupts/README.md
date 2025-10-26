## Interrupts

So far, we've touched a bunch of hardware on the MB2. We've read out buttons, waited for timers, done serial communication, and talked to devices using I2C.  Each of these things involved waiting for one or more peripherals to become ready. So far, our waiting was by "polling": repeatedly asking the peripheral if it's done yet, until it is.

Seeing as our microcontroller only has a single CPU core, it cannot do anything else while it waits. On top of that, a CPU core continuously polling a peripheral wastes power, and in a lot of applications, we can't have that. Can we do better?

Luckily, we can! While our little microcontroller can't compute things in parallel, it can easily switch between different tasks during execution, responding to events from the outside world. This switching is done using a feature called "interrupts".

Interrupts are aptly named: they allow peripherals to actually interrupt the core program execution at any point in time. On our MB2's nRF52833, peripherals are connected to the core's Nested Vectored Interrupt Controller (NVIC). The NVIC can stop the CPU in its tracks, instruct it to go do something else, and once that's done, get the CPU back to what it was doing before it was interrupted. We'll cover the Nested and Vectored parts of the interrupt controller later: let's first focus on how the core switches tasks.

### Handling Interrupts

The model of computation used by our NRF52833 is the one used by almost every modern CPU. Inside the CPU are "scratch-pad" storage locations known as "CPU registers". (Confusingly, these CPU registers are different from the "device registers" we discussed earlier in the [Registers] chapter.)  To carry out a computation, the CPU typically loads values from memory to CPU registers, performs the computation using the register values, then stores the result back to memory.  (This is known as a "load-store architecture".)

Everything about the computation the CPU is currently running is stored in the CPU registers. If the core is going to switch tasks, it must store the contents of the CPU registers somewhere so that the new task can use the registers as its own scratch-pad. When the new task is complete the CPU can then restore the register values and restart the old computation.  Sure enough, that is exactly the first thing the core does in response to an interrupt request: it stops what it's doing immediately and stores the contents of the CPU registers on the stack.

The next step is actually jumping to the code that should be run in response to an interrupt.  An Interrupt Service Routine (ISR), often referred to as an interrupt "handler", is a special function in your application code that gets called by the core in response to interrupts. An "interrupt table" in memory contains an "interrupt vector" for every possible interrupt: the interrupt vector indicates what ISR to call when a specific interrupt is received. We describe the details of ISR vectoring in the [NVIC and Interrupt Priority] section.

An ISR function "returns" using a special return-from-interrupt machine instruction that causes the CPU to restore the CPU registers and jump back to where it was before the ISR was called.

## Poke The MB2

Let's define an ISR and configure an interrupt to "poke" the MB2 when Button A is pressed
(`examples/poke.rs`). The board will respond by saying "ouch" and panicking.

```rust
{{#include examples/poke.rs}}
```

The ISR handler function is "special". The name `GPIOTE` is required here, indicating
that this ISR should be stored at the entry for the `GPIOTE` interrupt in the interrupt table.

The `#[interrupt]` decoration is used at compile time to mark a function to be treated specially as
an ISR. (This is a "proc macro": you can read more about it in the [Rust book] if you wish.)

Essentially, a "proc macro" translates source code into other source code. If you are curious as to what any particular macro use translates into,
you could expand that macro invocation. You can do this by using either the Tools in the [Rust Playground] or the "rust-analyzer: Expand macro" command in your IDE.

Marking a function with `#[interrupt]` implies several special things about the function:

* The compiler will check that the function takes no arguments and returns no value (or never returns). The CPU has no
  arguments to provide to an ISR, and no place to put a return value from the ISR. This is because interrupt handlers have their own call stack (at least *conceptually* if not always in practice).

* A vector to this function (that is a function pointer) will be placed at the location in the interrupt table
  which corresponds to the function's name.

* The compiler will prevent directly calling the ISR from normal code.

There are two steps to configure the interrupt. First, the GPIOTE must be set up to generate an
interrupt when the pin connected to Button A goes from high to low voltage. Second, the NVIC must be
configured to allow the interrupt. Order matters a bit: doing things in the "wrong" order may
generate an interrupt before you are ready to handle it.

**Note** As with most microcontrollers, there is a lot of flexibility in when the GPIOTE can generate an interrupt. Interrupts can be generated on low-to-high pin transition, high-to-low (as here), any change ("edge"), when low, or when high. On the nRF52833, interrupts generate an event that must be manually cleared in the ISR to ensure that the ISR is not called a second time for the same interrupt. Other microcontrollers may work a little differently — you should read Rust crate and microcontroller documentation to understand the details on a different board.

When you push the A Button, you will see an "ouch" message and then a panic. Why does the interrupt
handler call `panic!()`? Try commenting the `panic!()` call out and see what happens when you push
the button. You will see "ouch" messages scroll off the screen. The NVIC records when an interrupt
has been issued: that "event" is kept until it is explicitly cleared by the running program. Without
the `panic!()`, when the interrupt handler returns the NVIC will (in this case) re-enable the
interrupt, notice that there is still an interrupt event pending, and run the handler again. This
will continue forever: each time the interrupt handler returns it will be called again. As we will
see in a bit, the interrupt indication can be cleared from within the interrupt handler using the
`reset_event()` peripheral method.

You may define ISRs for many different interrupt sources: when I2C is ready, when a timer expires,
and on and on. Inside an ISR you can do pretty much anything you want, but it's good practice to
keep the interrupt handlers short and quick.

Normally, once an ISR is complete the main program continues running just as it would have if the interrupt had not happened. This is a bit of a problem, though: how does your application notice that the ISR has run and done things? Seeing as an ISR doesn't have any input parameters or result, how can ISR code interact with application code?

[NVIC and Interrupt Priority]: nvic-and-interrupt-priority.html
[Registers]: ../09-registers/index.html
[Rust Playground]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024
[Rust book]: https://doc.rust-lang.org/book/ch20-05-macros.html