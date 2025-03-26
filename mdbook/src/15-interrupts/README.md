## Interrupts

So far, we've gone though a fair bunch of topics about embedded software.  We've read out buttons,
waited for timers, done serial communication, and talked to other things on the Microbit board using
I2C.  Each of these things involved waiting for one or more peripherals to become ready. So far, our
waiting was by "polling": repeatedly asking the peripheral if it's done yet, until it is.

Seeing as our microcontroller only has a single CPU core, it cannot do anything else while it
waits. On top of that, a CPU core continuously polling a peripheral wastes power, and in a lot of
applications, we can't have that. Can we do better?

Luckily, we can. While our little microcontroller can't compute things in parallel, it can easily
switch between different tasks during execution, responding to events from the outside world. This
switching is done using a feature called "interrupts"!

Interrupts are aptly named: they allow peripherals to actually interrupt the core program execution
at any point in time. On our MB2's nRF52833, peripherals are connected to the core's Nested Vectored
Interrupt Controller (NVIC). The NVIC can stop the CPU in its tracks, instruct it to go do something
else, and once that's done, get the CPU back to what it was doing before it was interrupted. We'll
cover the Nested and Vectored parts of the interrupt controller later: let's first focus on how the
core switches tasks.

### Handling Interrupts

Computation is always contextual: the core always needs memory to load inputs and store outputs to.
Our microcontroller is of what's known as a load-store-architecture, and as such the core does not
store and load it's computation parameters and results in RAM directly.  Instead, our core has
access to a small amount scratch pad memory: the CPU registers.  Confusingly, these CPU registers
are different from the device registers we discussed earlier in the [Registers] chapter.

As far as the core is concerned, all context about the computation that it is doing is stored in the
CPU registers. If the core is going to switch tasks, it must store the contents of the CPU registers
somewhere, so that the new task can use them as their own scratchpad memory.  Sure enough, that is
exactly the first thing the core does in response to an interrupt request: it stops what it's doing
immediately and stores the contents of the CPU registers on the stack.

The next step is actually jumping to the code that should be run in response to an interrupt.
Interrupt Service Routines (ISRs), often referred to as interrupt handlers, are special functions in
your application code that get called by the core in response to specific interrupts. An ISR
function "returns" using a special return-from-interrupt machine instruction that causes the CPU to
restore the CPU registers and jump back to where it was before the ISR was called.

## Poke The MB2

Let's define an ISR and configure an interrupt to "poke" the MB2 when Button A is pressed
(`examples/poke.rs`). The board will respond by saying "ouch" and panicking.

```rust
{{#include examples/poke.rs}}
```

The ISR handler function is "special". The name `GPIOTE` is required here, and the function must be
decorated with `#[interrupt]` so that it returns using a return-from-interrupt instruction rather
than the normal way. The function may not take arguments and must return `()`.

There are two steps to configure the interrupt. First, the GPIOTE must be set up to generate an
interrupt when the wire connect to Button A goes from high to low voltage. Second, the NVIC must be
configured to allow the interrupt. Order matters a bit: doing things in the "wrong" order may
generate a bogus interrupt before you are ready to handle it.

When you push the A Button, you will see an "ouch" message and then a panic. Why does the interrupt
handler call `panic!()`? Try commenting the `panic!()` call out and see what happens when you push
the button. You will see "ouch" messages scroll off the screen. The GPIOTE records when an interrupt
has been issued, and that record is kept until it is explicitly cleared by the running
program. Without the `panic!()`, when the interrupt handler returns the GPIOTE will re-enable the
interrupt, notice that an interrupt has been issued and not cleared, and run the handler again. This
will continue forever: each time the interrupt handler returns it will be called again. In the next
section we will see how to clear the interrupt indication from within the interrupt handler.

You may define ISRs for many different interrupt sources: when I2C is ready, when a timer expires,
and on and on. Inside an ISR you can do pretty much anything you want, but it's good practice to
keep the interrupt handlers short and quick.

When the ISR function returns (using a magic instruction), the CPU looks to see if interrupts have
happened that need to be handled, and if so calls one of the handlers (according to a priority order
set by the NVIC). Otherwise, the CPU restores the CPU registers and returns to the running program
as if nothing has happened.

But if the core just goes on with its life after handling an interrupt, how does your device know
that it happened? Seeing as an ISR doesn't have any input parameters or result, how can ISR code
interact with application code?

[Registers]: https://docs.rust-embedded.org/discovery-mb2/07-registers
