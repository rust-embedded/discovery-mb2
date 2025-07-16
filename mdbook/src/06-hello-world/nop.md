# NOP

You might wonder what that `nop()` call is doing in the `wait()` loop in `src/bin/spin-wait.rs`.

The answer is that it literally does nothing. The `nop()` function causes the compiler to put a
`NOP` Arm machine instruction at that point in the program. `NOP` is a special instruction that
causes the CPU to skip it. To ignore it. To literally do No OPeration with it (hence the name).

So get rid of that line and recompile the program. Don't forget `--release` mode. Then run it.

We're back to a slightly darker solid LED again. With no loop body, the compiler's optimizer decided
that `wait()` function wasn't doing anything. So it just removed it for you at compile time. Thanks
optimizer. You have made my wait loop infinitely fast.

How does `nop()` do its job? Well, if you look at the implementation of `nop()` you will find
(after a bunch of digging around) that it is implemented like this:

```rust
asm!("nop", options(nomem, nostack, preserves_flags));
```

The `nop()` function is "inlined", so when you "call" it an actual Arm `NOP` assembly instruction is
inserted into your program's code at that point. Because details, this `NOP` will not be removed or
moved around by the compiler: it will stay right there where you put it.

The ability to insert assembly code into your program where needed is sometimes quite important in
embedded programming. Sometime a CPU will have instructions the compiler doesn't know about, but
that you still need in order to use the CPU effectively. Rust's `asm!()` directive gives you a way
to do that.

Our spin-wait is still terrible. Let's talk about doing better.
