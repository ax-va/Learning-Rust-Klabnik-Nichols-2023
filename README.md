# Learning-Rust-Klabnik-Nichols-2023

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

```unix
$ cargo --version
cargo 1.83.0 (5ffbef321 2024-10-29)
```

### Create a Cargo project

`cargo new` creates a new directory and project.

```unix
$ cd 01*/01-2*
$ cargo new hello_cargo
    Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello_cargo
```
The command creates `Cargo.toml`, `scr/main.rs` and a Git repo along with `.gitignore` 
if you do not run `cargo new` within an existing Git repo. 
To create a git repo anyway, use the `--vcs=git` flag.

Run `cargo new --help` to see the available options.

See more keys and their definitions in `Cargo.toml`:
https://doc.rust-lang.org/cargo/reference/manifest.html

### Build a Cargo project

Build the project:

```unix
.../hello_cargo$ cargo build
   Compiling hello_cargo v0.1.0 (path/to/project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
```

This creates an executable file `target/debug/hello_cargo`. Run it:
```unix
.../hello_cargo$ ./target/debug/hello_cargo
Hello, world!
```

### Build a Cargo project for release

Compile the project with optimizations into an executable in `target/release` instead of `target/debug`.

```unix
$ cargo build --release
```

### Build and run a Cargo project

It is *more convenient* to compile the code and then run the executable file by an all-in-one command.
If the source code hadn't changed, so Cargo didn't rebuild them but just ran the binary.
```unix
.../hello_cargo$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_cargo`
Hello, world!
```

### Check if a Cargo project is still compiling

Check your code to make sure it compiles without producing an executable.
Often, `cargo check` is much faster than `cargo build` because it skips the step of producing an executable.
```unix
.../hello_cargo$ cargo check
    Checking hello_cargo v0.1.0 (path/to/01--hello-rust/01-2--hello-cargo/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
```
