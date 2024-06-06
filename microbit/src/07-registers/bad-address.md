# `0xBAAAAAAD` address

Not all the peripheral memory can be accessed. Look at this program.

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
        ptr::read_volatile(0x5000_A784 as *const u32);
    }

    loop {}
}
```

This address is close to the `OUT` address we used before but this address is *invalid*.
Invalid in the sense that there's no register at this address.

Now, let's try it.

``` console
$ cargo run
(..)
Resetting and halting target
Target halted
(gdb) continue
Continuing.

Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:9
9	#[entry]
(gdb) continue
Continuing.

Program received signal SIGINT, Interrupt.
registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:10
10	fn main() -> ! {
(gdb) continue
Continuing.

Breakpoint 3, cortex_m_rt::HardFault_ (ef=0x2001ffb8) at src/lib.rs:1046
1046	    loop {}
(gdb) 
```

We tried to do an invalid operation, reading memory that doesn't exist, so the processor raised an
*exception*, a *hardware* exception.

In most cases, exceptions are raised when the processor attempts to perform an invalid operation.
Exceptions break the normal flow of a program and force the processor to execute an *exception
handler*, which is just a function/subroutine.

There are different kind of exceptions. Each kind of exception is raised by different conditions and
each one is handled by a different exception handler.

The `aux7` crate depends on the `cortex-m-rt` crate which defines a default
*hard fault* handler, named `HardFault_`, that handles the "invalid memory
address" exception. `embed.gdb` placed a breakpoint on `HardFault`; that's why
the debugger halted your program while it was executing the exception handler.
We can get more information about the exception from the debugger. Let's see:

```
(gdb) list
1040  #[allow(unused_variables)]
1041	#[doc(hidden)]
1042	#[cfg_attr(cortex_m, link_section = ".HardFault.default")]
1043	#[no_mangle]
1044	pub unsafe extern "C" fn HardFault_(ef: &ExceptionFrame) -> ! {
1045	    #[allow(clippy::empty_loop)]
1046	    loop {}
1047	}
1048	
1049	#[doc(hidden)]
1050	#[no_mangle]
```

`ef` is a snapshot of the program state right before the exception occurred. Let's inspect it:

```
(gdb) print/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x5000a784,
  r1: 0x3,
  r2: 0x2001ff24,
  r3: 0x0,
  r12: 0x1,
  lr: 0x4403,
  pc: 0x43ea,
  xpsr: 0x1000000
}
```

There are several fields here but the most important one is `pc`, the Program Counter register.
The address in this register points to the instruction that generated the exception. Let's
disassemble the program around the bad instruction.

```
(gdb) disassemble /m ef.pc
Dump of assembler code for function core::ptr::read_volatile<u32>:
1654	pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x000043d2 <+0>:	push	{r7, lr}
   0x000043d4 <+2>:	mov	r7, sp
   0x000043d6 <+4>:	sub	sp, #16
   0x000043d8 <+6>:	str	r0, [sp, #4]
   0x000043da <+8>:	str	r0, [sp, #8]

1655	    // SAFETY: the caller must uphold the safety contract for `volatile_load`.
1656	    unsafe {
1657	        assert_unsafe_precondition!(
   0x000043dc <+10>:	b.n	0x43de <core::ptr::read_volatile<u32>+12>
   0x000043de <+12>:	ldr	r0, [sp, #4]
   0x000043e0 <+14>:	movs	r1, #4
   0x000043e2 <+16>:	bl	0x43f4 <core::ptr::read_volatile::precondition_check>
   0x000043e6 <+20>:	b.n	0x43e8 <core::ptr::read_volatile<u32>+22>

1658	            check_language_ub,
1659	            "ptr::read_volatile requires that the pointer argument is aligned and non-null",
1660	            (
1661	                addr: *const () = src as *const (),
1662	                align: usize = align_of::<T>(),
1663	            ) => is_aligned_and_not_null(addr, align)
1664	        );
1665	        intrinsics::volatile_load(src)
   0x000043e8 <+22>:	ldr	r0, [sp, #4]
   0x000043ea <+24>:	ldr	r0, [r0, #0]          ; <-- That's the one!
   0x000043ec <+26>:	str	r0, [sp, #12]
   0x000043ee <+28>:	ldr	r0, [sp, #12]

1666	    }
1667	}
   0x000043f0 <+30>:	add	sp, #16
   0x000043f2 <+32>:	pop	{r7, pc}

End of assembler dump.

```

The exception was caused by the `ldr r0, [r0, #0]` instruction, a read instruction. The instruction
tried to read the memory at the address indicated by the `r0` *CPU register*. By the way, a CPU
(processor) register not a memory mapped register; it doesn't have an associated address like, say,
`OUT`.

Wouldn't it be nice if we could check what the value of the `r0` register was right at the instant
when the exception was raised? Well, we already did! The `r0` field in the `ef` value we printed
before is the value of `r0` register had when the exception was raised. Here it is again:

```
(gdb) print/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x5000a784,
  r1: 0x3,
  r2: 0x2001ff24,
  r3: 0x0,
  r12: 0x1,
  lr: 0x4403,
  pc: 0x43ea,
  xpsr: 0x1000000
}
```

`r0` contains the value `0x5000_A784` which is the invalid address we called the `read_volatile`
function with.
