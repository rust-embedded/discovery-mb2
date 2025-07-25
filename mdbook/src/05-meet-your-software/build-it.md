# Build it

The first step is to build our "binary" crate. Because the microcontroller has a different
architecture than your computer we'll have to cross compile. Cross compiling in Rust land is as
simple as passing an extra `--target` flag to `rustc`or Cargo. The complicated part is figuring out
the argument of that flag: the *name* of the target.

As we already know the microcontroller on the micro:bit v2 has a Cortex-M4F processor in it.
`rustc` knows how to cross-compile to the Cortex-M architecture and provides several different
targets that cover the different processors families within that architecture:

- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4**F** and Cortex-M7**F** processors
- `thumbv8m.main-none-eabi`, for the Cortex-M33 and Cortex-M35P processors
- `thumbv8m.main-none-eabihf`, for the Cortex-M33**F** and Cortex-M35P**F** processors

"Thumb" here refers to a version of the Arm instruction set that has smaller instructions for
reduced code size (it's a pun, see). The `hf`/`F` parts have hardware floating point
acceleration. This will make numeric computations involving fractional ("floating decimal point")
computations much faster.

For the micro:bit v2, we'll want the `thumbv7em-none-eabihf` target.

Before cross-compiling you have to download a pre-compiled version of the standard library (a
reduced version of it, actually) for your target. That's done using `rustup`:

``` console
$ rustup target add thumbv7em-none-eabihf
```

You only need to do the above step once; `rustup` will then update this target (re-installing a new
standard library `rust-std` component that contains the `core` library we use) whenever you update
your toolchain. Therefore you can skip this step if you have already added the necessary target
while [verifying your setup].

[verifying your setup]: ../03-setup/verify.html#verifying-cargo-embed


With the `rust-std` component in place you can now cross compile the program using Cargo.  Make sure
you are in the `mdbook/src/05-meet-your-software` directory in the Git repo, then build. This initial code
is an example, so we compile it as such.

``` console
$ cargo build --example init
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.86
   ...

    Finished dev [unoptimized + debuginfo] target(s) in 33.67s
```

> **NOTE** Be sure to compile this crate *without* optimizations. The provided `Cargo.toml` file and
> build command above will ensure optimizations are off as long as you *don't* pass `cargo` the
> `--release` flag.

OK, now we have produced an executable. This executable won't blink any LEDs: it's just a simplified
version that we will build upon later in the chapter.  As a sanity check, let's verify that the
produced executable is actually an Arm binary. (The command below is equivalent to

    readelf -h ../../../target/thumbv7em-none-eabihf/debug/examples/init

on systems that have `readelf`.)

``` console
$ cargo readobj --example init -- --file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           Arm
  Version:                           0x1
  Entry point address:               0x117
  Start of program headers:          52 (bytes into file)
  Start of section headers:          793112 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         4
  Size of section headers:           40 (bytes)
  Number of section headers:         21
  Section header string table index: 19
```

If your numbers don't exactly match these, don't worry: a lot of this is quite dependent
on the current build environment. 

Next, we'll flash the program into our microcontroller.
