/*
```
$ cd 11*
$ cd 11-1*
$ cargo new adder --lib
$ cd adder
```

Run all tests in the project

```
$ cargo test
   Compiling adder v0.1.0 (.../Learning-Rust-Klabnik-Nichols-2023/11--automated-tests/11-1/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/lib.rs (target/debug/deps/adder-cebaff100d82fd72)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test
   Compiling adder v0.1.0 (.../Learning-Rust-Klabnik-Nichols-2023/11--automated-tests/11-1/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/debug/deps/adder-cebaff100d82fd72)

running 2 tests
test tests::it_works ... ok
test tests::it_fails ... FAILED

failures:

---- tests::it_fails stdout ----
thread 'tests::it_fails' panicked at src/lib.rs:54:9:
Make this test fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_fails

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

See also:
- benchmark tets https://doc.rust-lang.org/unstable-book/library-features/test.html
 */

// function to test
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
// test module
mod tests {
    use super::*;

    // Indicate a test function, so the test runner
    // knows to treat this function as a test.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        // assert macro
        assert_eq!(result, 4);
    }

    // Add a test to fail
    #[test]
    fn it_fails() {
        panic!("Make this test fail");
    }
}
