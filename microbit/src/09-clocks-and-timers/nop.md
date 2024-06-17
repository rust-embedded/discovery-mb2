# NOP

If in the previous section you compiled the program in release mode and actually looked at the
disassembly, you probably noticed that the `delay` function is optimized away and never gets called
from within `main`.

LLVM decided that the function wasn't doing anything worthwhile and just removed it.

There is a way to prevent LLVM from optimizing the `for` loop delay: add a *volatile* assembly
instruction. Any instruction will do but NOP (No OPeration) is a particular good choice in this case
because it has no side effect.

Your `for` loop delay would become:

```rust
{{#include src/bin/nop.rs:7:13 }}
```

And this time `delay` won't be compiled away by LLVM when you compile your program in release mode:

``` console
$ cargo objdump --bin clocks-and-timers --release -- -d --no-show-raw-insn
clocks-and-timers:      file format ELF32-arm-little

Disassembly of section .text:
[...]
00000158 <clocks_and_timers::delay::h79d66d34776636ea>:
     158:      	push	{r4, r5, r7, lr}
     15a:      	add	r7, sp, #0x8
     15c:      	movs	r4, #0x0
     15e:      	adds	r5, r4, #0x1
     160:      	bl	0x1432 <__nop>          @ imm = #0x12ce
     164:      	cmp	r4, #0x95
     166:      	mov	r4, r5
     168:      	blo	0x15e <clocks_and_timers::delay::h79d66d34776636ea+0x6> @ imm = #-0xe
     16a:      	pop	{r4, r5, r7, pc}
[...]
```

Now, test this: Compile the program in debug mode and run it, then compile the program in release
mode and run it. What's the difference between them? What do you think is the main cause of the
difference? Can you think of a way to make them equivalent or at least more similar again?
