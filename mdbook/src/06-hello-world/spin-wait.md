# Spin wait

To blink the LED, we need to wait about a half-second between each change. How do we do that?

Well, here's the dumb way. It's not good, but it's a start. Take a look at `examples/spin-wait.rs`.

```rust
{{#include examples/spin-wait.rs}}
```

Run this with `cargo run --release --example spin-wait` — the `--release` is really important here — and
you should see the LED on your MB2 flash on and off *about* once per second.

Things you might be wondering:

* **What are those `_` characters in that number?** Rust allows these in numbers and ignores them.
  It's really convenient to make big numbers more readable. Here we are using them as commas (or
  whatever the separator is for groups of three digits in your country).

* **If the nRF52833 is running at 64MHz, why is the wait loop iterating only 4M times? Shouldn't it
  be 32M?** The wait loop executes several instructions each time through: the `nop` (see next
  section), some bookkeeping, and a branch back to the start of the loop. The code generated is
  roughly this for the first `wait()` call
  
  ```asm
  .LBB1_4:
      adds r3, #1
      nop
      cmp  r3, r2
      bne  .LBB1_4
  ```

  and this for the second

  ```asm
  .LBB1_6:
      subs	r3, #1
      nop
      bne	.LBB1_6
  ```

  This is only three or four instructions, but the backward branch may cost an extra bit.  Notice
  that these *are not the same:* the compiler chooses to emit different instructions for the first
  and second wait loops. See "it varies depending" below.
  
  Still, we're executing about 4 instructions per loop iteration. This means that on our 64MHz CPU a
  half-second spin should take 64M/2/4 = 8M iterations to complete. So something is slowing us down
  by a factor of 2. What? I dunno. This whole thing is terrible.

* **Why is `--release` so all-important?** Try without it. Notice that the LED is still flashing on
  and off, but with a period of *many* seconds. The wait loop is now unoptimized and is taking many
  instructions each time through.

* **What is that `nop()` call and why is it there?** We shall answer this in the next section.

* **Why do you refer to this as "the dumb way"?**

  * **It isn't precise.** Trying to tune that loop to reliably hit exactly 0.5 seconds is… not
    really a thing.

  * **It varies depending.** Different CPU? Different compilation flags? Different anything really?
    Now the timing has changed.

  * **It sucks power.** The CPU is running instructions as fast as it can, just to stay in place.
    If there's nothing else for it to do, it should quietly sleep until it is needed again. This
    doesn't matter much if you have USB power. But if you hook up your MB2 using the battery pack
    you'll really feel this.

In the next section, we'll discuss `nop()`. After that, we'll talk more about the other things about
our blinky that need improving.

For such a simple program, this is a pretty complicated program. That's why we start with blinky.
