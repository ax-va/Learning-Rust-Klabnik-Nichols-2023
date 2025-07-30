# Cargo

## Cargo Workspaces

A *workspace* is a set of packages that share a top-level `Cargo.lock` file and `target` directory
to help manage multiple related packages developed in tandem.

As your project grows, consider using a workspace. 
It helps break your code into smaller, more understandable components, 
rather than managing one large, monolithic codebase. 
Additionally, grouping crates in a workspace simplifies coordination - 
especially when multiple crates are frequently updated together.

### Example

- Create the binary crate by running `cargo new add` and then the top-level workspace `Cargo.toml` 

    ```
    |-- adder
        |-- Cargo.toml
        |-- src
            |-- main.rs
    |-- Cargo.toml
    ```

- In the workspace `Cargo.toml`, use `[workspace]` instead of `[package]` specifying the path to the package
    ```toml
    [workspace]
    members = [
        "adder",
    ]
    ```

- Build the workspace
    ```unix
    $ cargo build
    ```

    that creates
    ```
    |-- adder
        |-- src
            |-- main.rs
        |-- Cargo.toml
    |-- target
    |-- Cargo.lock
    |-- Cargo.toml
    ```

    The workspace has one `target` directory at the top level that the compiled artifacts will be placed into.

- Create library crates by running `cargo new add_one --lib` and `cargo new add_two --lib`
    ```
    |-- add_one
        |-- src
            |-- lib.rs
        |-- Cargo.toml
    |-- add_two
        |-- src
            |-- lib.rs
        |-- Cargo.toml
    |-- adder
        |-- src
            |-- main.rs
        |-- Cargo.toml
    |-- target
    |-- Cargo.lock
    |-- Cargo.toml
    ```
    The top-level `Cargo.toml` file will be updated automatically to
    ```toml
    [workspace]
    members = ["add_one", "add_two",
        "adder",
    ]
    ```

- Add path dependencies to `adder/Cargo.toml`

    ```toml
    [dependencies]
    add_one = { path = "../add_one" }
    add_two = { path = "../add_two" }
    ```

- Use the both library crates in `adder/src/main.rs`
    ```rust
    use add_one;
    use add_two;
    
    fn main() {
        let num = 10;
        println!("{num} plus one is {}.", add_one::add_one(num));
        println!("{num} plus two is {}.", add_two::add_two(num));
    }
    ```

- Build the workspace by running `cargo build` in the top-level project directory.

- Run the package using the `-p` argument followed by the package name
    ```unix
    $ cargo run -p adder
    ```

- Add the dependency on the external `rand` crate to `add_one` in `add_one/Cargo.toml`
    ```toml
    [dependencies]
    rand = "0.8.5"
    ```
  We can now use `rand` in `add_one` by adding `use rand;`, but not in `add_two` and `adder`. 
  Shared dependencies can be given in the `[workspace.dependencies]` section in the root `Cargo.toml` file.

- Build the whole workspace again to bring in and compile the `rand` crate for the `add_one` crate
    ```unix
    $ cargo build
    ```

### Tests in Workspaces

- Running `cargo test` in a top-level workspace directory
run the tests for all the crates in the workspace
    ```unix
    $ cargo test
    ```

- Run tests for one particular crate in a workspace from the top-level directory by using the `-p` flag
    ```unix
    $ cargo test -p add_one
    ```

### Publishing crates

If you publish the crates in the workspace to https://crates.io, 
each crate in the workspace will need to be published separately.