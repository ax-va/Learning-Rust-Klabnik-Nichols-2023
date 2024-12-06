## Create a Cargo project

`cargo new` creates a new directory and project.

```unix
$ cd 01--hello-rust/01-2--hello-cargo
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

## Build and run a Cargo project

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

It is *more convenient* to compile the code and then run the executable file by an all-in-one command.
```unix
.../hello_cargo$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_cargo`
Hello, world!
```
