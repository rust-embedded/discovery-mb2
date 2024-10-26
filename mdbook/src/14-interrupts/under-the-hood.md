
## Under The Hood

We've seen that interrupts make our processor immediately jump to another function in the code, but
what's going on behind the scenes to allow this to happen? In this section we'll cover some
technical details that won't be necessary for the rest of the book, so feel free to skip ahead if
you're not interested.

### The Interrupt Controller

Interrupts allow the processor to respond to peripheral events such as a GPIO input pin changing
state, a timer completing its cycle, or a UART receiving a new byte. The peripheral contains
circuitry that notices the event and informs a dedicated interrupt-handling peripheral. On Arm
processors, the interrupt-handling peripheral is called the NVIC — the Nested Vector Interrupt
Controller.

> **NOTE** On other microcontroller architectures such as RISC-V the names and details discussed
> here will differ, but the underlying principles are generally very similar.

The NVIC can receive requests to trigger an interrupt from many peripherals. It's even common for a
peripheral to have multiple possible interrupts, for example a GPIO port having an interrupt for
each pin, or a UART having both a "data received" and "data finished transmission" interrupt. The
job of the NVIC is to prioritise these interrupts, remember which ones still need to be procesed,
and then cause the processor to run the relevant interrupt handler code.

Depending on its configuration, the NVIC can ensure the current interrupt is fully processed before
a new one is executed, or it can stop the processor in the middle of one interrupt in order to
handle another that's higher priority.  This is called "preemption" and allows processors to respond
very quickly to critical events.  For example, a robot controller might use low-priority interrupts
to keep track sending status information to the operator, but also have a high-priority interrupt to
detect an emergency stop button being pushed so it can immediately stop moving the motors. You
wouldn't want it to wait until it had finished sending a data packet to get around to stopping!

In embedded Rust, we can program the NVIC using the [`cortex-m`] crate, which provides methods to
enable and disable (called `unmask` and `mask`) interrupts, set interrupt priorities, and trigger
interrupts from software. Frameworks such as [RTIC] can handle NVIC configuration for you, taking
advantage of the NVIC's flexibility to provide convenient resource sharing and task management.

You can read more information about the NVIC in [Arm's documentation].

[`cortex-m`]: https://docs.rs/cortex-m/latest/cortex_m/peripheral/struct.NVIC.html
[RTIC]: https://rtic.rs/
[Arm's documentation]: https://developer.arm.com/documentation/ddi0337/e/Nested-Vectored-Interrupt-Controller/About-the-NVIC

### The vector table

When describing the NVIC, I said it could "cause the processor to run the relevant interrupt handler
code". But how does that actually work?

First, we need some way for the processor to know which code to run for each interrupt. On Cortex-M
processors, this involves a part of memory called the vector table. It is typically located at the
very start of the flash memory that contains our code, which is reprogrammed every time we upload
new code to our processor, and contains a list of addresses -- the locations in memory of every
interrupt function. The specific layout of the start of memory is defined by Arm in the
[Architecture Reference Manual]; for our purposes the important part is that bytes 64 through to 256
contain the addresses of all 48 interrupt handlers for the nRF processor we use, four bytes per
address. Each interrupt has a number, from 0 to 47. For example, `TIMER0` is interrupt number 8, and
so bytes 96 to 100 contain the four-byte address of its interrupt handler. When the NVIC tells the
processor to handle interrupt number 8, the CPU reads the address stored in those bytes and jumps
execution to it.

How is this vector table generated in our code? We use the [`cortex-m-rt`] crate which handles this
for us. It provides a default interrupt for every unused position (since every position must be
filled) and allows our code to override this default whenever we want to specify our own interrupt
handler. We do this using the `#[interrupt]` macro, which requires that our function be given a
specific name related to the interrupt it handles. Then the `cortex-m-rt` crate uses its linker
script to arrange for the address of that function to be placed in the right part of memory.

For more details on how these interrupt handlers are managed in Rust, see the [Exceptions] and
[Interrupts] chapters in the Embedded Rust Book.

[Architecture Reference Manual]: https://developer.arm.com/documentation/ddi0403/latest
[`cortex-m-rt`]: https://docs.rs/cortex-m-rt
[Exceptions]: https://docs.rust-embedded.org/book/start/exceptions.html
[Interrupts]: https://docs.rust-embedded.org/book/start/interrupts.html
