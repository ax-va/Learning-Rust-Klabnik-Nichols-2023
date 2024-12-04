# Learning-Rust-Klabnik-Nichols-2023

## Source

https://github.com/rust-lang/book

## Install Rust through rustup on Ubuntu

**rustup** is a command line tool for managing Rust versions and associated tools.

```unix
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/delorian/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /home/delorian/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/delorian/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/delorian/.profile
  /home/delorian/.bashrc

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
> <Enter>
...
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
```
If you get *linker errors*, you should install a C compiler, which will typically include a linker.
Rust uses the linker to join its compiled outputs into one file.

Reload the shell configuration to update the `PATH` environment variable.

```unix
$ rustc --version
rustc 1.83.0 (90b35a623 2024-11-26)
```

## Update or uninstall Rust with rustup 

```unix
$ rustup update
```

```unix
$ rustup self uninstall
```
