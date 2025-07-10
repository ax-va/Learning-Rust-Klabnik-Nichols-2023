# Learning-Rust-Klabnik-Nichols-2023

## Why to program in Rust

- On the one hand, Rust does not have manual memory management like in C or C++, and thanks to its ownership and borrowing system, the code becomes safe both in terms of security vulnerabilities and typical memory errors.
- On the other hand, Rust has no garbage collector - memory is freed deterministically, which makes the compiled code very fast.

## Original source code

https://github.com/rust-lang/book

## Install Rust through rustup on Linux or macOS

**rustup** is a command line tool for managing Rust versions and associated tools.

```unix
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:
...
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
If you get *linker errors*, you should install a C compiler, which will typically include a *linker*.
Rust uses the linker to join its compiled outputs into one file.

Reload the shell configuration to update the `PATH` environment variable.

Check if you have Rust installed:

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

## Use the local documentation

Run `rustup doc` to open the local documentation in your browser:

```unix
$ rustup doc
Opening docs in your browser
```

## Compile a source code and run an executable

Rust is an *ahead-of-time compiled* language meaning compilation and execution 
are explicitly separate steps, unlike, say, Python or JavaScript.

- On Linux and macOS, you will have two files: `main` (an executable file) and  `main.rs` (an open-source file).
```unix
$ cd <path/to/directory>
$ rustc main.rs
$ ls
main  main.rs
$ ./main
Hello, world!
```

- On Windows, you will have three files: 
`main.exe`, `main.pdb` (for debugging information), and `main.rs`.
```windows
> cd <path/to/directory>
> rustc main.rs
> .\main.exe 
```

## Cargo

*Cargo* is Rust's build system and package manager. 
It comes installed with Rust if you used the official installers.

See also `rust-summary/cargo.md` for more information.
