## Sharing Data With Globals

> **NOTE:** This content is partially taken from
> <https://onevariable.com/blog/interrupts-is-threads/>, which contains more discussion about this
> topic.

As we mentioned in the last section, when an interrupt occurs we aren't passed any arguments. How do
we get access to things needed in the interrupt handler, such as the peripherals or other main
program state?

### Std Rust: Sharing Data With A Thread

In "std" Rust, we also have to think about sharing data when we do things like
spawn a thread.

When you want to *give* something to a thread, you might pass it
by ownership.

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

If we want to *share* something, and still have access to it in the original thread,
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

We need to *make sure the data lives long enough* for both the current thread and the new thread we
are creating. We can do this by putting it in an `Arc` (Atomically Reference Counted heap
allocation) like this:

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

Why does "std" Rust make us do this? Rust is helping us out by making us think about two things:

1. The data lives long enough (potentially "forever"!)
2. Only one piece of code can mutably access the data at the same time

If Rust allowed us to access data that might not live long enough, like data borrowed from one
thread into another, things might go wrong. We might get corrupted data if the original thread ends
or panics and then the second thread tries to access the data that is now invalid. If Rust allowed
two pieces of code to access the same data at the same, we could have a data race, or the data could
end up corrupted.

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

> **TODO AJM:** Next talk about how statics only (safely) allow read access, we need
inner mutability to get write access, show something with a mutex'd integer that
we can init in const context

> **TODO AJM:** THEN talk about data that doesn't exist at startup, like sticking a
peripheral in after being configured, and how we do that, something like Lazy
Use Bart's crate for now, maybe add Lazy to the Blocking Mutex crate?
