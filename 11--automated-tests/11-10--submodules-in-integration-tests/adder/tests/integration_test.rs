/*
Instead of creating `tests/common.rs`, create `tests/common/mod.rs`.
Then the section in the test output corresponding to the `common` module will no longer appear.
Naming the file this way tells Rust not to treat the `common` module as an integration test *file*.
```
adder
|-- Cargo.lock
|-- Cargo.toml
|-- src
    |-- lib.rs
|-- tests
    |-- common
        |-- mod.rs
    |-- integration_test.rs
```
 */

// The `adder` crate is the current crate.
// But in integration tests, it is treated like an external crate,
// because integration tests simulate using our crate from the outside.
use adder;
// Include the local test helper module
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}