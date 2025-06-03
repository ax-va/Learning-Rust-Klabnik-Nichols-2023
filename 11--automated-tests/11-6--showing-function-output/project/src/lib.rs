/*
```
$ cd 11*
$ cd 11-6*
$ cargo new project --lib
$ cd project
```

By default, the `println!` output will not be printed for passing tests.
If a test fails, we will see whatever was printed to standard output
for the failed test with the rest of the failure message.

Run all the tests in the project in parallel
```
$ cargo test
...
running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
Value: 7. <----------------------------------------------------- SHOWED HERE
thread 'tests::this_test_will_fail' panicked at src/lib.rs:40:9:
assertion `left == right` failed
  left: 7
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

Set the option to see printed values for passing tests as well.

```unix
$ cargo test -- --show-output
...
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

successes:

---- tests::this_test_will_pass stdout ----
Value: 5. <----------------------------------------------------- SHOWED HERE


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
Value: 7. <----------------------------------------------------- SHOWED HERE
thread 'tests::this_test_will_fail' panicked at src/lib.rs:67:9:
assertion `left == right` failed
  left: 7
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```
 */

fn print_value_and_return_10(a: i32) -> i32 {
    println!("Value: {a}.");
    10
}

#[cfg(test)]
mod tests {
    // Because the tests module is an inner module,
    // we need to bring the code under test into the scope of the inner module.
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = print_value_and_return_10(5);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = print_value_and_return_10(7);
        assert_eq!(7, value);
    }
}
