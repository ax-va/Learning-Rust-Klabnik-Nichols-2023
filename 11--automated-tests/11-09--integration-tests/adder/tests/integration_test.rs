/*
*Integration tests* are entirely external to your library
and use your code in the same way any other external code would,
using only the public interface and potentially exercising multiple modules per test.
Their purpose is to test whether many parts of your library work together correctly.
The reason for integration tests is that units of code
that work correctly on their own could have problems when integrated.

Create a `tests` directory at the top level of our project directory, next to `src`.
Each file in the `tests` directory is a separate crate.
```
adder
|-- Cargo.lock
|-- Cargo.toml
|-- src
    |-- lib.rs
|-- tests
    |-- integration_test.rs
```
Cargo treats the `tests` directory specially
and compiles files in this directory only when we run `cargo test`.

Run tests.
```
$ cargo test
   Compiling adder v0.1.0 (/path/to/Learning-Rust-Klabnik-Nichols-2023/11--automated-tests/11-9--integration-tests/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running unittests src/lib.rs (target/debug/deps/adder-cebaff100d82fd72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-02cf4dc97b288673)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
The three sections of output include the unit tests, the integration test, and the doc tests.
If any test in a section fails, the following sections will not be run.

Run all the tests in the `tests/integration_test.rs` file.
```
$ cargo test --test integration_test
   Compiling adder v0.1.0 (/path/to/Learning-Rust-Klabnik-Nichols-2023/11--automated-tests/11-9--integration-tests/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running tests/integration_test.rs (target/debug/deps/integration_test-02cf4dc97b288673)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
 */

// The `adder` crate is the current crate.
// But in integration tests, it is treated like an external crate,
// because integration tests simulate using our crate from the outside.
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}