## Interrupts

So far, we've gone though a fair bunch of topics about
embedded software.  We've read out buttons, waited for
timers, done serial communication, and talked to other
things on the Microbit board using I2C.  Each of these
things involved waiting for one or more peripherals to
become ready. So far, our waiting was by "polling":
repeatedly asking the peripheral if it's done yet, until it
is.

Seeing as our microcontroller only has a single CPU core, it
cannot do anything else while it waits. On top of that, a
CPU core continuously polling a peripheral wastes power, and
in a lot of applications, we can't have that. Can we do
better?

Luckily, we can. While our little microcontroller can't
compute things in parallel, it can easily switch between
different tasks during execution, responding to events from
the outside world. This switching is done using a feature
called "interrupts"!

Interrupts are aptly named: they allow peripherals to
actually interrupt the core program execution at any point
in time. On our MB2's nRF52833, peripherals are connected to
the core's Nested Vectored Interrupt Controller (NVIC). The
NVIC can stop the CPU in its tracks, instruct it to go do
something else, and once that's done, get the CPU back to
what it was doing before it was interrupted. We'll cover the
Nested and Vectored parts of the interrupt controller later:
let's first focus on how the core switches tasks.

### Handling Interrupts

Computation is always contextual: the core always needs memory to load inputs and store outputs to.
Our microcontroller is of what's known as a load-store-architecture, and as such
the core does not store and load it's computation parameters and results in RAM directly.
Instead, our core has access to a small amount scratch pad memory: the core registers.
Note that, confusingly, these core registers are different from the registers we've discussed in chapter 7.

As far as the core is concerned, all context about the computation that it is doing is stored
in the core registers. If the core is going to switch tasks, it must store the contents
of the core registers somewhere, so that the other task can use them as their own scratchpad memory.
And that is exactly the first thing the core does in response to an interrupt request:
it stops what it's doing immediately and stores the contents of the core registers on the stack.

The next step is actually jumping to the code that should be run in response to an interrupt.
Interrupt Service Routines (ISRs), often referred to as interrupt handlers,
are special sections in your application code that get executed by the core
in response to specific interrupts.

## Example with panicking goes here!

Here's an example of some code that defines an ISR and configures an interrupt:
```rust
/* Timer goes off and program goes BOOM example */
```

In case of our microcontroller, you may
define an ISR that gets executed when I2C is ready, and another one that gets
executed in response to a button press. Inside an ISR you can do pretty much
anything you want, but it's good practice to keep the interrupt handlers
short and quick.

Once the ISR is done (NOTE: Done automatically on return), the core loads back the original content of its core
registers and returns to the point where it left off, almost as if nothing happened.

But if the core just goes on with its life after handling an interrupt, how does
your device know that it happened? And seeing as an ISR doesn't have any input parameters,
how can ISR code interact with application code?

> Note to @hdoordt: Please "hand off"/end here by making the point
> that interrupts don't take/return anything, so `fn() -> ()` or
> `void func(void)`, or let me know so I can change the intro of
> the next section! -James


## James: Interlude about sharing

> Note: Stealing from https://onevariable.com/blog/interrupts-is-threads/

As we mentioned in the last section, when an interrupt occurs we aren't passed
any arguments, so how do we get access to things like the peripherals, or other
information we might need?

### How you do it in desktop rust

> * Spawn a thread (pass data in)
>     * By ownership
>     * With Arc
>     * With ArcMutex
> * Have some kind of globals
>     * With ArcMutex
>     * With Lazy

In "desktop" Rust, we also have to think about sharing data when we do things like
spawn a thread. When you want to *give* something to a thread, you might pass it
by ownership:

```rust
// Create a string in our current thread
let data = String::from("hello");

// Now spawn a new thread, and GIVE it ownership of the string
// that we just created
std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("{data}");
});
```

If we want to SHARE something, and still have access to it in the original thread,
we usually can't pass a reference to it. If we do this:

```rust
use std::{thread::{sleep, spawn}, time::Duration};

fn main() {
    // Create a string in our current thread
    let data = String::from("hello");
    
    // make a reference to pass along
    let data_ref = &data;
    
    // Now spawn a new thread, and GIVE it ownership of the string
    // that we just created
    spawn(|| {
        sleep(Duration::from_millis(1000));
        println!("{data_ref}");
    });
    
    println!("{data_ref}");
}
```

We get an error like this:

```text
error[E0597]: `data` does not live long enough
  --> src/main.rs:6:20
   |
3  |       let data = String::from("hello");
   |           ---- binding `data` declared here
...
6  |       let data_ref = &data;
   |                      ^^^^^ borrowed value does not live long enough
...
10 | /     spawn(|| {
11 | |         sleep(Duration::from_millis(1000));
12 | |         println!("{data_ref}");
13 | |     });
   | |______- argument requires that `data` is borrowed for `'static`
...
16 |   }
   |   - `data` dropped here while still borrowed
```

We need to **make sure the data lives long enough** for both the current thread and the
new thread we are creating. We can do this by putting it in an `Arc`, or an Atomically
Reference Counted heap allocation, like this:

```rust
use std::{sync::Arc, thread::{sleep, spawn}, time::Duration};

fn main() {
    // Create a string in our current thread
    let data = Arc::new(String::from("hello"));
    
    let handle = spawn({
        // Make a copy of the handle, that we GIVE to the new thread.
        // Both `data` and `new_thread_data` are pointing at the
        // same string!
        let new_thread_data = data.clone();
        move || {
            sleep(Duration::from_millis(1000));
            println!("{new_thread_data}");
        }
    });
    
    println!("{data}");
    // wait for the thread to stop
    let _ = handle.join();
}
```

This is great! We can now access the data in both the main thread as long as we'd
like. But what if we want to *mutate* the data in both places?

For this, we usually need some kind of "inner mutability", a type that doesn't
require an `&mut` to modify. On the desktop, we'd typically reach for a type
like a `Mutex`, which requires us to `lock()` it before we can gain mutable access
to the data.

That might look something like this:

```rust
use std::{sync::{Arc, Mutex}, thread::{sleep, spawn}, time::Duration};

fn main() {
    // Create a string in our current thread
    let data = Arc::new(Mutex::new(String::from("hello")));
    
    // lock it from the original thread
    {
        let guard = data.lock().unwrap();
        println!("{guard}");
        // the guard is dropped here at the end of the scope!
    }
    
    let handle = spawn({
        // Make a copy of the handle, that we GIVE to the new thread.
        // Both `data` and `new_thread_data` are pointing at the
        // same `Mutex<String>`!
        let new_thread_data = data.clone();
        move || {
            sleep(Duration::from_millis(1000));
            {
                let mut guard = new_thread_data.lock().unwrap();
                // we can modify the data!
                guard.push_str(" | thread was here! |");                
                // the guard is dropped here at the end of the scope!
            }
        }
    });
    
    // wait for the thread to stop
    let _ = handle.join();
    {
        let guard = data.lock().unwrap();
        println!("{guard}");
        // the guard is dropped here at the end of the scope!
    }
}
```

If we run this code, we get:

```text
hello
hello | thread was here! |
```

### Why does desktop rust make us do this?

Rust is helping us out by making us think about two things:

1. The data lives long enough (potentially "forever"!)
2. Only one piece of code can mutably access the data at the same time

If Rust allowed us to access data that might not live long enough, like data borrowed
from one thread into another, we might get corrupted data if the original thread
ends or panics, and the second thread tries to access the data that is now invalid.

If Rust allowed two pieces of code to access the same data at the same, we could have
a data race, or the data could end up corrupted.

### What's the same in embedded rust?

In embedded Rust we care about the same things when it comes to sharing data with
interrupts! Similar to threads, interrupts can occur at any time, sort of like
a thread waking up and accessing some shared data. This means that the data we
share with an interrupt must live long enough, and we must be careful to ensure
that our main code isn't in the middle of accessing some data shared with the
interrupt, just to have the interrupt run and ALSO access that data!

In fact, in embedded Rust, we model interrupts in almost exactly the same way
that threads are modeled in Rust, meaning that the same rules apply, for the
same reasons.

### What's different in embedded rust?

However, in embedded Rust, we have some crucial differences:

Interrupts don't work exactly like threads: we set them up ahead of time, and
they wait until some event happens (like a button being pressed, or a timer
expiring), at which point they run, but without access to any context.

They can also be triggered multiple times, once for each time that the event
occurs.

Since we can't pass context to interrupts as arguments like a function, we
need to find another place to store that data.

Additionally, in many cases we don't have access to heap allocations, that
are used by things like `Arc` above to store our data.

Without the ability to pass things by value, and without a heap to store data,
that leaves us with one place to put our shared data that an interrupt can
access: `static`s.

TODO AJM: Next talk about how statics only (safely) allow read access, we need
inner mutability to get write access, show something with a mutex'd integer that
we can init in const context

TODO AJM: THEN talk about data that doesn't exist at startup, like sticking a
peripheral in after being configured, and how we do that, something like Lazy
Use Bart's crate for now, maybe add Lazy to the Blocking Mutex crate?

## Working With Interrupts: Blinky Button

## Under the hood

We've seen that interrupts make our processor immediately jump to another
function in the code, but what's going on behind the scenes to allow this to
happen? In this section we'll cover some technical details that won't be
necessary for the rest of the book, so feel free to skip ahead if you're not
interested.

### The interrupt controller

Interrupts allow the processor to respond to peripheral events such as a GPIO
input pin changing state, a timer completing its cycle, or a UART receiving a
new byte. The peripheral contains circuitry that notices the event and informs
a dedicated interrupt-handling peripheral. On Arm processors, this is called
the NVIC -- the nested vector interrupt controller.

> **NOTE** On other microcontroller architectures such as RISC-V, the names and
> details discussed here might differ, but the underlying principles are
> generally very similar.

The NVIC can receive requests to trigger an interrupt from many peripherals,
and it's even common for a peripheral to have multiple possible interrupts, for
example a GPIO having an interrupt for each pin, or a UART having both a "data
received" and "data finished transmission" interrupt. Its job is to prioritise
these interrupts, remember which ones still need to be procesed, and then cause
the processor to run the relevant interrupt handler code.

Depending on its configuration, the NVIC can ensure the current interrupt is
fully processed before a new one is executed, or it can stop the processor in
the middle of one interrupt in order to handle another that's higher priority.
This is called "pre-emption" and allows processors to respond very quickly to
critical events.  For example, a robot controller might use low-priority
interrupts to keep track sending status information to the operator, but also
have a high-priority interrupt to detect an emergency stop button being pushed
so it can immediately stop moving the motors. You wouldn't want it to wait
until it had finished sending a data packet to get around to stopping!

In embedded Rust, we can program the NVIC using the [`cortex-m`] crate, which
provides methods to enable and disable (called `unmask` and `mask`) interrupts,
set their priorities, and manually trigger them. Frameworks such as [RTIC] can
handle NVIC configuration for you, taking advantage of its flexibility to
provide convenient resource sharing and task management.

You can read more information about the NVIC in [Arm's documentation].

[`cortex-m`]: https://docs.rs/cortex-m/latest/cortex_m/peripheral/struct.NVIC.html
[RTIC]: https://rtic.rs/
[Arm's documentation]: https://developer.arm.com/documentation/ddi0337/e/Nested-Vectored-Interrupt-Controller/About-the-NVIC

### The vector table

When describing the NVIC, I said it could "cause the processor to run the
relevant interrupt handler code". But how does that actually work?

First, we need some way for the processor to know which code to run for each
interrupt. On Cortex-M processors, this involves a part of memory called the
vector table. It is typically located at the very start of the flash memory
that contains our code, which is reprogrammed every time we upload new code to
our processor, and contains a list of addresses -- the locations in memory of
every interrupt function. The specific layout of the start of memory is defined
by Arm in the [Architecture Reference Manual]; for our purposes the important
part is that bytes 64 through to 256 contain the addresses of all 48 interrupts
in the nRF processor we use, four bytes per address. Each interrupt has a
number, from 0 to 47. For example, `TIMER0` is interrupt number 8, and so bytes
96 to 100 contain the four-byte address of its interrupt handler. When the NVIC
tells the processor to handle interrupt number 8, the CPU reads the address
stored in those bytes and jumps execution to it.

How is this vector table generated in our code? We use the [`cortex-m-rt`]
crate which handles this for us. It provides a default interrupt for every
unused position (since every position must be filled), and allows our code to
override this default whenever we want to specify our own interrupt handler. We
do this using the `#[interrupt]` macro, which causes our function to be given a
specific name related to the interrupt it handles. Finally, the `cortex-m-rt`
crate uses its linker script to arrange for the address of that function to be
placed in the right part of memory.

For more details on how these interrupt handlers are managed in Rust, see the
[Exceptions] and [Interrupts] chapters in the Embedded Rust Book.

[Architecture Reference Manual]: https://developer.arm.com/documentation/ddi0403/latest
[`cortex-m-rt`]: https://docs.rs/cortex-m-rt
[Exceptions]: https://docs.rust-embedded.org/book/start/exceptions.html
[Interrupts]: https://docs.rust-embedded.org/book/start/interrupts.html
