# (mis)Optimization

Reads/writes to registers are quite special. I may even dare to say that they are embodiment of side
effects. In the previous example we wrote four different values to the same register. If you didn't
know that address was a register, you may have simplified the logic to just write the final value `0x00000000` into the register.

Actually, LLVM, the compiler's backend / optimizer, does not know we are dealing with a register and
will merge the writes thus changing the behavior of our program. Let's check that really quick.

First, we'll use cargo objdump to get us the assembly of the build artifacts from both the optimized and the non-optimized build.

```
# Non-optimized
cargo objdump -- --disassemble --no-show-raw-insn --source > debug.dump
# Optimized
cargo objdump --release -- --disassemble --no-show-raw-insn --source > release.dump
```

Let's see what's in there. Specifically, let's try to find the assembly that manipulates the `OUT` register.

First, let's have a look at the contents of `debug.dump`, the assembly from the non-optimized build.
I skipped a bunch and added my comments behind the `; <--`, indicating the line number in the source code that corresponds
to the instruction.

```
$ cat debug.dump
[...]
00000158 <main>:
     158:      	push	{r7, lr}
     15a:      	mov	r7, sp
     15c:      	bl	0x160 <registers::__cortex_m_rt_main::h0b7888ca966441cf> @ imm = #0x0

00000160 <registers::__cortex_m_rt_main::h0b7888ca966441cf>:
     160:      	push	{r7, lr}
     162:      	mov	r7, sp
     164:      	sub	sp, #0x8
     166:      	bl	0x198 <aux7::init::hb6346637538e8ec5> @ imm = #0x2e
     16a:      	movw	r1, #0x504        ; <-- Load lower half of `OUT` register address into register `r1`
     16e:      	movt	r1, #0x5000       ; <-- Load upper half of `OUT` register address into register `r1`
     172:      	str	r1, [sp, #0x4]
     174:      	ldr	r0, [r1]          ; <-- (16) Load value at the address in `r1` into `r0`.
     176:      	orr	r0, r0, #0x200000 ; <-- (16) Bitwise OR the value in `r0` with `0x200000`, and store in `r0`
     17a:      	str	r0, [r1]          ; <-- (16) Store contents of `r0` in memory at address from `r1`
     17c:      	ldr	r0, [r1]          ; <-- (19) Load value at the address in `r1` into `r0`.
     17e:      	orr	r0, r0, #0x80000  ; <-- (19) Bitwise OR the value in `r0` with `0x80000`, and store in `r0`
     182:      	str	r0, [r1]          ; <-- (19) Store contents of `r0` in memory at address from `r1`
     184:      	ldr	r0, [r1]          ; <-- (22) Load value at the address in `r1` into `r0`.
     186:      	bic	r0, r0, #0x200000 ; <-- (22) Bitwise AND the value in `r0` with bitwise complement of `0x200000`, and store in `r0`
     18a:      	str	r0, [r1]          ; <-- (22) Store contents of `r0` in memory at address from `r1`
     18c:      	ldr	r0, [r1]          ; <-- (25) Load value at the address in `r1` into `r0`.
     18e:      	bic	r0, r0, #0x80000  ; <-- (25) Bitwise AND the value in `r0` with bitwise complement of `0x80000`, and store in `r0`
     192:      	str	r0, [r1]          ; <-- (25) Store contents of `r0` in memory at address from `r1`
     194:      	b	0x196 <registers::__cortex_m_rt_main::h0b7888ca966441cf+0x36> @ imm = #-0x2
     196:      	b	0x196 <registers::__cortex_m_rt_main::h0b7888ca966441cf+0x36> @ imm = #-0x4
[...]
```

As you can see, the non-optimized assembly contains 4 loads, 4 stores, and 4 bit manipulation instructions.
Those correspond nicely with the code we wrote. Now, let's have a look at the optimized assembly.

```
$ cat release.dump
[...]
00000158 <main>:
     158:      	push	{r7, lr}
     15a:      	mov	r7, sp
     15c:      	bl	0x160 <registers::__cortex_m_rt_main::h1f38525e07b97485> @ imm = #0x0

00000160 <registers::__cortex_m_rt_main::h1f38525e07b97485>:
     160:      	push	{r7, lr}
     162:      	mov	r7, sp
     164:      	bl	0x17a <aux7::init::h4390f1d4f8a071f7> @ imm = #0x12
     168:      	movw	r0, #0x504          ; <-- Load lower half of `OUT` register address into register `r0`
     16c:      	movt	r0, #0x5000         ; <-- Load upper half of `OUT` register address into register `r0`
     170:      	ldr	r1, [r0]                ; <-- (?) Load value at the address in `r0` into `r1`.
     172:      	bic	r1, r1, #0x280000       ; <-- (?) Bitwise AND the value in `r1` with bitwise complement of `0x280000`, and store in `r1`
     176:      	str	r1, [r0]                ; <-- (?) Store contents of `r0` in memory at address from `r0`
     178:      	b	0x178 <registers::__cortex_m_rt_main::h1f38525e07b97485+0x18> @ imm = #-0x4
[...]
```

Huh? Just a single load - bit manipulate - store?
The state of the LEDs didn't change this time! The `str` instruction is the one that writes a value
to the register. Our *debug* (unoptimized) program had four of them, one for each write to the
register, but the *release* (optimized) program only has one.

How do we prevent LLVM from misoptimizing our program? We use *volatile* operations instead of plain
reads/writes:

``` rust
#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::entry;

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A magic address!
        const PORT_P0_OUT: u32 = 0x50000504;

        // Turn on the top row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out | 1 << 21);

        // Turn on the bottom row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out | 1 << 19);

        // Turn off the top row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out & !(1 << 21));

        // Turn off the bottom row
        let out = ptr::read_volatile(PORT_P0_OUT as *mut u32);
        ptr::write_volatile(PORT_P0_OUT as *mut u32, out & !(1 << 19));
    }

    loop {}
}
```

Let's run cargo objdump once again, with optimizations enabled.

```
cargo objdump --release -- --disassemble --no-show-raw-insn  > release.volatile.dump
```

All right, now have a look at what's inside:

```
$ cat release.volatile.dump
[...]
00000158 <main>:
     158:      	push	{r7, lr}
     15a:      	mov	r7, sp
     15c:      	bl	0x160 <registers::__cortex_m_rt_main::h1f38525e07b97485> @ imm = #0x0

00000160 <registers::__cortex_m_rt_main::h1f38525e07b97485>:
     160:      	push	{r7, lr}
     162:      	mov	r7, sp
     164:      	bl	0x192 <aux7::init::h4390f1d4f8a071f7> @ imm = #0x2a
     168:      	movw	r0, #0x504
     16c:      	movt	r0, #0x5000
     170:      	ldr	r1, [r0]
     172:      	orr	r1, r1, #0x200000
     176:      	str	r1, [r0]
     178:      	ldr	r1, [r0]
     17a:      	orr	r1, r1, #0x80000
     17e:      	str	r1, [r0]
     180:      	ldr	r1, [r0]
     182:      	bic	r1, r1, #0x200000
     186:      	str	r1, [r0]
     188:      	ldr	r1, [r0]
     18a:      	bic	r1, r1, #0x80000
     18e:      	str	r1, [r0]
     190:      	b	0x190 <registers::__cortex_m_rt_main::h1f38525e07b97485+0x30> @ imm = #-0x4
[...]
```

Hey, look at that! Now we've got our four load - manipulate - store cycles back.
Step through the code once again using GDB to see the volatile operations in action!
