## Sharing Data With Globals

> **NOTE:** This content is partially taken with permission from the blog post
> *[Interrupts Is Threads]* by James Munns, which contains more discussion about this
> topic.

As I mentioned in the last section, when an interrupt occurs we aren't passed any arguments and
cannot return any result. This makes it hard for our program interact with peripherals and other
main program state. Before worrying about this
bare-metal embedded problem, it is likely worth thinking about threads in "std" Rust.

### "std" Rust: Sharing Data With A Thread

In "std" Rust, we also have to think about sharing data when we do things like
spawn a thread.

When you want to *give* something to a thread, you might pass it into a closure by ownership.

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

If you want to *share* something, and still have access to it in the original thread, you usually
can't pass a reference to it. If you do this:

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

you'll get an error like this:

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

You need to *make sure the data lives long enough* for both the current thread and the new thread
you are creating. You can do this by putting it in an `Arc` (Atomically Reference Counted heap
allocation) like this:

```rust
use std::{sync::Arc, thread::{sleep, spawn}, time::Duration};

fn main() {
    // Create a string in our current thread
    let data = Arc::new(String::from("hello"));
    
    let handle = spawn({
        // Make a copy of the handle to GIVE to the new thread.
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

This is great! You can now access the data in both the main thread as long as you'd
like. But what if you want to *mutate* the data in both places?

For this, you will usually need some kind of "inner mutability" — a type that doesn't require an
`&mut` to modify. On the desktop, you'd typically reach for a type like `Mutex`, `lock()`-ing it to
gain mutable access to the data.

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
        // Make a copy of the handle, that you GIVE to the new thread.
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

If you run this code, you will see:

```text
hello
hello | thread was here! |
```

Why does "std" Rust make us do this? Rust is helping us out by making us think about two things:

1. The data lives long enough (potentially "forever"!)
2. Only one piece of code can mutably access the data at a time

If Rust allowed us to access data that might not live long enough, like data borrowed from one
thread into another, things might go wrong. We might get corrupted data if the original thread ends
or panics and then the second thread tries to access the data that is now invalid. If Rust allowed
two pieces of code to try to mutate the same data at the same, we could have a data race, or the
data could end up corrupted.

### Embedded Rust: Sharing Data With An ISR

In embedded Rust we care about the same things when it comes to sharing data with interrupt
handlers! Similar to threads, interrupts can occur at any time, sort of like a thread waking up and
accessing some shared data. This means that the data we share with an interrupt must live long
enough, and we must be careful to ensure that our main code isn't in the middle of accessing some
data shared with the interrupt, just to have the interrupt run and ALSO access that data!

In fact, in embedded Rust, we model interrupts in a similar way that we model threads in Rust: the
same rules apply, for the same reasons. However, in embedded Rust, we have some crucial differences:

* Interrupts don't work exactly like threads: we set them up ahead of time, and they wait until some
  event happens (like a button being pressed, or a timer expiring). At that point they run, but
  without access to any context.

* Interrupts can be triggered multiple times, once for each time that the event occurs.

Since we can't pass context to interrupts as function arguments, we need to find another place to
store that data. In "bare metal" embedded Rust we don't have access to heap allocations: thus `Arc`
and similar are not possibilities for us.

Without the ability to pass things by value, and without a heap to store data, that leaves us with
one place to put our shared data that our ISR can access: `static` globals.

### Embedded Rust ISR Data Sharing: The "Standard Method" 

Global variables are very much second-class citizens in Rust, with many limitations compared to
local variables. You can declare a global state variable like this:

```rust
static COUNTER: usize = 0;
```

Of course, this isn't super-useful: you want to be able to mutate the `COUNTER`. You can
say 

```rust
static mut COUNTER: usize = 0;
```

but now all accesses will be unsafe.

```rust
unsafe { COUNTER += 1 };
```

The unsafety here is for a reason: imagine that in the middle of updating `COUNTER` an interrupt
handler runs and also tries to update `COUNTER`. The usual chaos will ensue. Clearly some kind of
locking is in order.

The `critical-section` crate provides a sort of `Mutex` type, but with an unusual API and unusual
operations. Examining the `Cargo.toml` for this chapter, you will see the feature
`critical-section-single-core` on the `cortex-m` crate enabled. This feature asserts that there is
only one processor core in this system, and that thus synchronization can be performed by simply
*disabling interrupts* across the critical section. If not in an interrupt, this will ensure that
only the main program has access to the global. If in an interrupt, this will ensure that the main
program cannot be accessing the global (program control is in the interrupt handler) and that no
other higher-priority interrupt handler can fire.

`critical_section::Mutex` is a bit weird in that it gives mutual exclusion but does not itself give
mutability. To make the data mutable, you will need to protect an interior-mutable type — usually
`RefCell` — with the mutex. This `Mutex` is also a bit weird in that you don't `.lock()`
it. Instead, you initiate a critical section with a closure that receives a "critical section token"
certifying that other program execution is prevented. This token can be passed to the `Mutex`'s
`borrow()` method to allow access.

Putting it all together gives us the ability to share state between ISRs and the main program
(`examples/count-once.rs`).

```rust
{{#include examples/count-once.rs}}
```

We still cannot safely return from our ISR, but now we are in a position to do something about that:
share the `GPIOTE` with the ISR so that the ISR can clear the interrupt.

> **TODO AJM:** THEN talk about data that doesn't exist at startup, like sticking a
peripheral in after being configured, and how we do that, something like Lazy
Use Bart's crate for now, maybe add Lazy to the Blocking Mutex crate?

[Interrupts Is Threads]: https://onevariable.com/blog/interrupts-is-threads
