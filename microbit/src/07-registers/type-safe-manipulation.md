# Type safe manipulation

One of the registers of `P0`, the `IN` register, is documented as a read-only register.

> 6.8.2.4 IN

Note that in the 'Access' column of the table, only the 'R' is given for this register.
We are not supposed to write to this register or Bad Stuff May Happen.

Registers have different read/write permissions. Some of them are write
only, others can be read and written to and there must be others that are read only.

Finally, directly working with hexadecimal addresses is error prone. You already saw that trying to
access an invalid memory address causesunreached class freedom porcupine shelf sensitize onion viscosity an exception which disrupts the execution of our program.

Wouldn't it be nice if we had an API to manipulate registers in a "safe" manner? Ideally, the API
should encode these three points I've mentioned: No messing around with the actual addresses, should
respect read/write permissions and should prevent modification of the reserved parts of a register.

Well, we do! `aux7::init()` actually returns a value that provides a type safe API to manipulate the
registers of the `P0` and `P1` ports.

As you may remember: a group of registers associated to a peripheral is called register block, and
it's located in a contiguous region of memory. In this type safe API each register block is modeled
as a `struct` where each of its fields represents a register. Each register field is a different
newtype over e.g. `u32` that exposes a combination of the following methods: `read`, `write` or
`modify` according to its read/write permissions. Finally, these methods don't take primitive values
like `u32`, instead they take yet another newtype that can be constructed using the builder pattern
and that prevent the modification of the reserved parts of the register.

The best way to get familiar with this API is to port our running example to it.

```rust
{{#include src/bin/type-safe.rs}}
```

First thing you notice: There are no magic addresses involved. Instead we use a more human friendly
way, `p0.out`, to refer to the `OUT` register in the `P0` port register block.

Then we have this [`modify`] method that takes a closure. Before this closure is called, the `OUT` register's value is read and passed to the closure as the `r` parameter. Given the value of `r`, you can manipulate `w` to the desired new value of the register using its methods. The result is the written to the register once the closure returns. In our case, the current value of the register is also passed in the `w` parameter, allowing us to just manipulate `w` when we want to keep the rest of the register bits as is.

The `modify` method is defined for registers that allow both write and read access. If you'd like to just read a register's value, but not update it, you can use the [`read`] method. Or, if you simply want to write a register value without reading, there's the [`write`] method.

Read-only registers only expose `read`, and write-only registers only expose `write`. This prevents users from accessing a register in a way that's not allowed, and therefore you don't need to wrap the calls in an `unsafe` block. And you don't need to figure out the exact register address and bit positions yourself!

[`write`]: https://docs.rs/svd2rust/latest/svd2rust/#write
[`read`]: https://docs.rs/svd2rust/latest/svd2rust/#read
[`modify`]: https://docs.rs/svd2rust/latest/svd2rust/#modify

Let's run this program! There's some interesting stuff we can do *while* debugging the program.

`p0` is a reference to the `P0` port's register block. `print p0` will return the base address of
the register block, and `print *p0` will print its value.

```
$ cargo run
(..)
Target halted
(gdb) set mem inaccessible-by-default off
(gdb) break main.rs:12
Breakpoint 4 at 0x162: main.rs:12. (2 locations)
(gdb) continue
Continuing.

Program received signal SIGINT, Interrupt.
cortex_m_rt::DefaultPreInit () at src/lib.rs:1058
1058	pub unsafe extern "C" fn DefaultPreInit() {}
(gdb) continue
Continuing.

Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:7
7	#[entry]
(gdb) continue
Continuing.

Program received signal SIGINT, Interrupt.
registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:8
8	fn main() -> ! {
(gdb) continue
Continuing.

Breakpoint 4.2, registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:12
12	    p0.out.modify(|_, w| w.pin21().set_bit());
(gdb) print *p0                                               ; ⬅️ Printing `*p0` here!
$1 = nrf52833_pac::p0::RegisterBlock {
  _reserved0: [0 <repeats 1284 times>],
  out: nrf52833_pac::generic::Reg<nrf52833_pac::p0::out::OUT_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::out::OUT_SPEC>
  },
  outset: nrf52833_pac::generic::Reg<nrf52833_pac::p0::outset::OUTSET_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::outset::OUTSET_SPEC>
  },
  outclr: nrf52833_pac::generic::Reg<nrf52833_pac::p0::outclr::OUTCLR_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::outclr::OUTCLR_SPEC>
  },
  in_: nrf52833_pac::generic::Reg<nrf52833_pac::p0::in_::IN_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::in_::IN_SPEC>
  },
  dir: nrf52833_pac::generic::Reg<nrf52833_pac::p0::dir::DIR_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 3513288704
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::dir::DIR_SPEC>
  },
  dirset: nrf52833_pac::generic::Reg<nrf52833_pac::p0::dirset::DIRSET_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 3513288704
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::dirset::DIRSET_SPEC>
  },
  dirclr: nrf52833_pac::generic::Reg<nrf52833_pac::p0::dirclr::DIRCLR_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 3513288704
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::dirclr::DIRCLR_SPEC>
  },
  latch: nrf52833_pac::generic::Reg<nrf52833_pac::p0::latch::LATCH_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::latch::LATCH_SPEC>
  },
  detectmode: nrf52833_pac::generic::Reg<nrf52833_pac::p0::detectmode::DETECTMODE_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::detectmode::DETECTMODE_SPEC>
  },
  _reserved9: [0 <repeats 472 times>],
  pin_cnf: [nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    } <repeats 11 times>, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
--Type <RET> for more, q to quit, c to continue without paging--c
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }]
}


```

All these newtypes and closures sound like they'd generate large, bloated programs but, if you
actually compile the program in release mode with [LTO] enabled, you'll see that it produces exactly
the same instructions that the "unsafe" version that used `write_volatile` and hexadecimal addresses
did!

[LTO]: https://en.wikipedia.org/wiki/Interprocedural_optimization

Use `cargo objdump` to grab the assembler code to `release.type-safe.dump`:
``` console
cargo objdump -q --release --bin type-safe -- --disassemble --no-show-raw-insn  > release.type-safe.dump
```

Then search for `main` in `release.type-safe.dump`
```
00000158 <main>:
     158:      	push	{r7, lr}
     15a:      	mov	r7, sp
     15c:      	bl	0x160 <registers::__cortex_m_rt_main::h0e9b57c6799332fd> @ imm = #0x0

00000160 <registers::__cortex_m_rt_main::h0e9b57c6799332fd>:
     160:      	push	{r7, lr}
     162:      	mov	r7, sp
     164:      	bl	0x192 <aux7::init::hec71dddc40be11b5> @ imm = #0x2a
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
     190:      	b	0x190 <registers::__cortex_m_rt_main::h0e9b57c6799332fd+0x30> @ imm = #-0x4
```

You can validate that this yields the exact same binary as the one with the calls to `ptr::read_volatile` and `ptr::write_volatile`.

The best part of all this is that nobody had to write a single line of code to implement the
GPIO API. All the code was automatically generated from a System View Description (SVD) file using the
[svd2rust] tool. This SVD file is actually an XML file that microcontroller vendors provide and that
contains the register maps of their microcontrollers. The file contains the layout of register
blocks, the base addresses, the read/write permissions of each register, the layout of the
registers, whether a register has reserved bits and lots of other useful information.

[svd2rust]: https://crates.io/crates/svd2rust
