# macOS

All the tools can be installed using [Homebrew]:

[Homebrew]: http://brew.sh/

``` console
$ # GDB debugger - The version in brew is built for all architectures including all of the ARM embedded cores
$ brew install gdb

$ # Minicom
$ brew install minicom

$ # lsusb lists USB ports
$ brew install lsusb
```

Unlike Linux, macOS does not require `udev` rules to access the micro:bit.

## Verify the tools

Check that Homebrew installed each tool:

``` console
$ gdb --version
$ minicom --version
$ lsusb -V
```

Rust should be managed by `rustup`, as described in the previous section. If
Rust was also installed with Homebrew, the Homebrew binaries may take priority
and prevent `rustup target add` from configuring the toolchain used by Cargo.
Check which binaries your shell finds:

``` console
$ type -a rustup rustc cargo
```

The first result for each command should be under `$HOME/.cargo/bin`. If
`rustc` or `cargo` instead resolves to `/opt/homebrew/bin` or
`/usr/local/bin`, remove the Homebrew Rust package and open a new terminal:

``` console
$ brew uninstall rust
```

## Verify the micro:bit connection

Connect the micro:bit using a USB cable and confirm that it appears with USB
vendor ID `0d28` and product ID `0204`:

``` console
$ lsusb -d 0d28:0204
```

The command should print a line describing the board. If it prints nothing,
try another USB cable or port; some cables provide power but do not carry
data.

Continue to the [next section] to check that `probe-rs` can communicate with
the board and flash a test program.

[next section]: verify.md
