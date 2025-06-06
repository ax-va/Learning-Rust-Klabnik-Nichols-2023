# Automated Tests

## Test Organization

### Unit Tests

The purpose of *unit tests* is to test individual components in isolation, 
using mocks or stubs to simulate external dependencies.
Unit tests should be fast and deterministic.

In Rust: put unit tests in the `src` directory in each file with the code 
that the unit tests are testing, and annotate the module with `#[cfg(test)]`.

The `#[cfg(test)]` annotation on the tests module tells Rust 
to compile and run the test code only when you run `cargo test`, 
not when you run `cargo build`.
This saves compile time and saves space in the resultant compiled artifact when you run `cargo build`.

The attribute `cfg` stands for configuration and tells Rust 
that the following item should only be included given a certain configuration option.
By using the `cfg` attribute, Cargo compiles our test code only if we actively run the tests with `cargo test`.
This includes any helper functions that might be within this module, 
in addition to the functions annotated with `#[test]`.

#### Testing Private Functions

Rust allows us to test private functions because `tests` is a child module 
and items in child modules can use the items in their ancestor modules by using `use super::*;`.

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// private function
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Because the tests module is an inner module,
    // we need to bring the code under test into the scope of the inner module.
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

### Integration Tests

*Integration tests* are entirely external to your library 
and use your code in the same way any other external code would, 
using only the public interface and potentially exercising multiple modules per test.
Their purpose is to test whether many parts of your library work together correctly.
The reason for integration tests is that units of code 
that work correctly on their own could have problems when integrated.

Create a `tests` directory at the top level of our project directory, next to `src`.
Each file in the `tests` directory is a separate crate.
```
project
|-- Cargo.lock
|-- Cargo.toml
|-- src
    |-- lib.rs
|-- tests
    |-- integration_test.rs
```
Cargo treats the `tests` directory specially 
and compiles files in this directory only when we run cargo test.

Run tests.
```
$ cargo test
```
The three sections of output include the unit tests, the integration test, and the doc tests.
If any test in a section fails, the following sections will not be run.

Run all the tests in the `tests/integration_test.rs` file.
```
$ cargo test --test integration_test
```

#### Submodules in Integration Tests

Instead of creating `tests/common.rs`, create `tests/common/mod.rs`.
Then the section in the test output corresponding to the `common` module will no longer appear.
Naming the file this way tells Rust not to treat the `common` module as an integration test *file*.
```
project
|-- Cargo.lock
|-- Cargo.toml
|-- src
    |-- lib.rs
|-- tests
    |-- common
        |-- mod.rs
    |-- integration_test.rs
```

#### No Integration Tests for Binary Crates

If our project is a binary crate that only contains a `src/main.rs` file and doesn't have a `src/lib.rs file`, 
we cannot create integration tests in the `tests` directory 
and bring functions defined in the `src/main.rs` file into scope with a `use` statement.
