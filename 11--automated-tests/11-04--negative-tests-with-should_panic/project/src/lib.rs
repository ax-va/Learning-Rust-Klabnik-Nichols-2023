/*
```
$ cd 11*
$ cd 11-04*
$ cargo new project --lib
$ cd project
```

Run all the tests in the project in parallel
```
$ cargo test
...
running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests project

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
 */

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // Because the tests module is an inner module,
    // we need to bring the code under test into the scope of the inner module.
    use super::*;

    #[test]
    // The test passes
    // 1. if any part of the function panics
    // 2. and the panic message contains the `expected` string.
    #[should_panic(expected="less than or equal to 100")]
    fn greater_than_100() {
        // Does panic.
        Guess::new(101); // Execution stops.
        // The test passes.
        Guess::new(100); // This code is never run, so it has no effect on the test result.
    }
}
