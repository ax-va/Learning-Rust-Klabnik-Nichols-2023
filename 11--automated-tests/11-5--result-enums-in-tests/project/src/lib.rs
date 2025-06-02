/*
```
$ cd 11*
$ cd 11-5*
$ cargo new project --lib
$ cd project
```

Run all the tests in the project in parallel
```
$ cargo test
```
 */

// function to test
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    // Because the tests module is an inner module,
    // we need to bring the code under test into the scope of the inner module.
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        assert_eq!(result, 4);

        // Rewrite to use `Result<T, E>`
        // and return an `Err` instead of panicking.
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("Error: two plus two does not equal four."))
        }
        // 1. Using `Result<T, E>` allows to use the question mark operator in the body of tests,
        // which can be a convenient way to write tests
        // that should fail if any operation within them returns an `Err` variant.
        // 2. We cannot use the `#[should_panic]` annotation on tests that use `Result<T, E>`.
        // To assert that an operation returns an `Err` variant,
        // don't use the question mark operator on the `Result<T, E>` value.
        // Instead, use `assert!(value.is_err())`.
    }
}
