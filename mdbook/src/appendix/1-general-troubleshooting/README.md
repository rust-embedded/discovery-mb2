# General troubleshooting

## `cargo-embed` problems

Most `cargo-embed` problems are related to not having installed the `udev` rules properly on
Linux, so make sure you got that right.

If you are stuck, you can open an issue in the [`discovery` issue tracker] or visit the [Rust
Embedded matrix channel] or the [probe-rs matrix channel] and ask for help there.

[`discovery` issue tracker]: https://github.com/rust-embedded/discovery-mb2/issues
[Rust Embedded matrix channel]: https://matrix.to/#/#rust-embedded:matrix.org
[probe-rs matrix channel]: https://matrix.to/#/#probe-rs:matrix.org

## Cargo problems

### "can't find crate for `core`"

*Symptoms:*

```
   Compiling volatile-register v0.1.2
   Compiling rlibc v1.0.0
   Compiling r0 v0.1.0
error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
Build failed, waiting for other jobs to finish...
error: Could not compile `r0`.

To learn more, run the command again with --verbose.
```

*Cause:*

You forgot to install the proper target for your microcontroller `thumbv7em-none-eabihf`.

*Fix:*

Install the proper target.

``` console
$ rustup target add thumbv7em-none-eabihf
```

### Unable to flash the device: `No loadable segments were found in the ELF file`

*Symptoms:*
```console
> cargo embed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
      Config default
      Target /home/user/embedded/target/thumbv7em-none-eabihf/debug/examples/init
 WARN probe_rs::flashing::loader: No loadable segments were found in the ELF file.
       Error No loadable segments were found in the ELF file.
```

*Cause:*

Cargo needs to know how to build and link the program to the requirements of the target device.
You therefore need to set the correct parameters in the `.cargo/config.toml` file.

*Fix:*

Add a `.cargo/config.toml` file with the correct parameters:
```toml
{{#include ../../05-meet-your-software/.cargo/config.toml}}
```

See [Embedded Setup](../../05-meet-your-software/embedded-setup.md) for further details.
