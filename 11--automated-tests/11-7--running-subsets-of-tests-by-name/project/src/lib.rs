/*
```
$ cd 11*
$ cd 11-7*
$ cargo new project --lib
$ cd project
```

We can choose which tests to run by passing `cargo test`
the name or names of the test(s) we want to run as an argument.

- Run a single test. The other tests were filtered out.
```
$ cargo test one_hundred
...
running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

- Run multiple tests: run only the tests with `add` in the name and filtered out the other tests.
```
$ cargo test add
...
running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

- Run all the test in the `tests` module.
```
$ cargo test tests
...
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
 */

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    // Because the tests module is an inner module,
    // we need to bring the code under test into the scope of the inner module.
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
