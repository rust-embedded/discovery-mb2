# Setting up a development environment

Dealing with microcontrollers involves several tools as we'll be dealing with an architecture
different from your computer's and we'll have to run and debug programs on a "remote" device.

## Documentation

Tooling is not everything though. Without documentation, it is pretty much impossible to work with
microcontrollers. The official MB2 technical documentation is at <https://tech.microbit.org>. We
will reference other technical documentation throughout the book.

## Tools

We'll use all the tools listed below. Where a minimum version is not specified, any recent version
should work but we have listed the version we have tested.

- Rust 1.79.0 or a newer toolchain.

- `gdb-multiarch`. This is a debugging tool. The oldest tested version is10.2, but other versions
  will most likely work as well.  If your distribution/platform does not have `gdb-multiarch`
  available `arm-none-eabi-gdb` will do the trick as well. Furthermore, some normal `gdb` binaries
  are built with multiarch capabilities as well: you can find further information about this in the
  debugging chapter of this book.

- [`cargo-binutils`]. Version 0.3.6 or newer.

  [`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils

- [`probe-rs-tools`]. Version 0.24.0 or newer.

  [`probe-rs-tools`]: https://probe.rs/docs/overview/about-probe-rs/

- `minicom` on Linux and macOS. Tested version: 2.7.1. Other versions will most likely work as well
  though.

- `PuTTY` on Windows.

Next, follow OS-agnostic installation instructions for a few of the tools:

### `rustc` & Cargo

Install rustup by following the instructions at [https://rustup.rs](https://rustup.rs).

If you already have rustup installed, double check that you are on the stable channel and your
stable toolchain is up-to-date. `rustc -V` should return a date and version no older than the one
shown below:

``` console
$ rustc -V
rustc 1.79.0 (129f3b996 2024-06-10)
```

### `cargo-binutils`

``` console
$ rustup component add llvm-tools
$ cargo install cargo-binutils --vers '^0.3'
$ cargo size --version
cargo-size 0.3.6
```

### `probe-rs-tools`

**NOTE** If you already have old versions of `probe-run`, `probe-rs` or `cargo-embed` installed
on your system, remove them before starting this step, as they could conceivably cause problems
for you down the line. In particular, `probe-run` no longer officially exists. Try these as
needed:

```console
$ cargo uninstall cargo-embed
$ cargo uninstall probe-run
$ cargo uninstall probe-rs
$ cargo uninstall probe-rs-cli
```

In order to install `probe-rs-tools`, go to <https://probe.rs> and follow the current installation
instructions there.

* **NOTE** If you prefer to install `probe-rs-tools` using `cargo install`, you can try the
  following steps.  Folks have experienced frequent failures with this approach, but you are
  welcome to give it a go.

  1. Upgrade to the most recent stable Rust.

  2. Install the `probe-rs-tools` binary
     [prerequisites](https://probe.rs/docs/getting-started/installation/).  (The linked
     instructions are part of the more general [`probe-rs`](https://probe.rs/) embedded debugging
     toolkit documentation.)

  3. Try the install

     ```console
     $ cargo install --locked probe-rs-tools
     ```

Installing `probe-rs-tools` will install several useful tools, including `probe-rs` and
`cargo-embed` (which is normally run as a Cargo command). Check that things are working before
proceeding.

```
$ cargo embed --version
cargo-embed 0.24.0 (git commit: crates.io)
```

### This repository

This book also contains some small Rust codebases used in various chapters: the easiest way to use
these is to download the book's source code. You can do this in one of the following ways:

- Visit the [repository](https://github.com/rust-embedded/discovery-mb2/), click the green "Code"
  button and then the "Download Zip" one.

- Clone it using `git` (if you know `git` you presumably already have it installed) from the same
  repository as linked in the Zip approach.

### OS specific instructions

Now follow the instructions specific to the OS you are using:

- [Linux](linux.md)
- [Windows](windows.md)
- [macOS](macos.md)
