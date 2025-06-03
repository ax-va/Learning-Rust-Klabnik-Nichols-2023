# Cargo

*Cargo* is Rust's build system and package manager. 
It comes installed with Rust if you used the official installers.

```unix
$ cargo --version
cargo 1.83.0 (5ffbef321 2024-10-29)
```

## Create a Cargo project

- `cargo new <project>` creates a `<project>` directory with `main.rs`;

- `cargo new <project> --lib` creates a `<project>` directory with `lib.rs`.

```unix
$ 01*/01-2*
$ cargo new hello_cargo
    Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello_cargo
```
The command creates `Cargo.toml`, `scr/main.rs` and a Git repo along with `.gitignore` 
if you don't run `cargo new` within an existing Git repo. 
To create a git repo anyway, use the `--vcs=git` flag.

Run `cargo new --help` to see the available options.

See more keys and their definitions in `Cargo.toml`:
https://doc.rust-lang.org/cargo/reference/manifest.html

## Build a Cargo project

Build the project:

```unix
.../hello_cargo$ cargo build
   Compiling hello_cargo v0.1.0 (path/to/project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
```

This command created an executable file `target/debug/hello_cargo`. 

Run it:
```unix
.../hello_cargo$ ./target/debug/hello_cargo
Hello, world!
```

## Build a Cargo project for release

Compile the project with optimizations into an executable in `target/release` instead of `target/debug`.

```unix
$ cargo build --release
```

## Build and run a Cargo project

It is *more convenient* to compile the code and then run the executable file by an all-in-one command.
If the source code hadn't changed, so Cargo didn't rebuild them but just ran the binary.
```unix
.../hello_cargo$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_cargo`
Hello, world!
```

## Check if a Cargo project is still compiling

Check your code to make sure it compiles without producing an executable.
Often, `cargo check` is much faster than `cargo build` because it skips the step of producing an executable.
```unix
.../hello_cargo$ cargo check
    Checking hello_cargo v0.1.0 (path/to/01--hello-rust/01-2--hello-cargo/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
```

## Cargo.lock

When you build a project for the first time, Cargo figures out all the versions of the dependencies 
that fit the criteria and then writes them to the `Cargo.lock` file. 
When you build your project in the future, Cargo will see that the `Cargo.lock` file exists and 
will use the versions specified there rather than doing all the work of figuring out versions again. 
This lets you have a *reproducible build* automatically.
Because the `Cargo.lock` file is important for reproducible builds, 
it's often checked into source control with the rest of the code in your project.

## Update crates

```unix
$ cargo update
```

This command will ignore the `Cargo.lock` file and figure out all the latest versions 
that fit your specifications in `Cargo.toml`. 
Cargo will then write those versions to the `Cargo.lock` file.

## Cargo for tests

Compile code in test mode and run the resultant test binary to execute 
all the tests *in parallel* and capture output generated during test runs in the test results.

```unix
$ cargo test 
```

Some command line options go to `cargo test`, and some go to the resultant test binary:

- `cargo test --help` displays the options to use with `cargo test`;

- `cargo test -- --help` displays the options to use with the resultant test binary.

### Running tests in parallel or consecutively

By default, all the tests run in parallel. 
Set the number of threads to the test binary.

```unix
$ cargo test -- --test-threads=1
```

Running the tests using one thread will take longer than running them in parallel, 
but the tests will not interfere with each other if they share state.

### Showing function output

By default, the `println!` output will not be printed for passing tests.
If a test fails, we will see whatever was printed to standard output 
in the failed test with the rest of the failure message.

Set the option to see printed values for passing tests as well.

```unix
$ cargo test -- --show-output
```
