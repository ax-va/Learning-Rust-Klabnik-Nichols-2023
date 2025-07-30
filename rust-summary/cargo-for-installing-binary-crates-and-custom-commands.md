# Crates

## Installing Binary Crates

The `cargo install` command allows you to install and use *binary* crates locally.
All binaries installed with cargo install are stored in the installation root's bin folder.

## Custom Commands

If a binary in your `$PATH` is named `cargo-something`, 
you can run it as if it were a Cargo subcommand by running `cargo something`.
Custom commands like this are also listed when you run `cargo --list`.
