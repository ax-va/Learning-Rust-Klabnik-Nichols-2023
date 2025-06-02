/*
```
$ cd 11*
$ cd 11-3*
$ cargo new project --lib
$ cd project
```

Run all the tests in the project in parallel
```
$ cargo test
...
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at src/lib.rs:53:9:
Greeting did not contain name, the actual greeting is `Hello!`.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
 */

pub fn greeting_v1(name: &str) -> String {
    // The `format!` macro returns a `String`,
    // which is an owned, growable, heap-allocated UTF-8 string.
    format!("Hello {name}!")
}

pub fn greeting_v2(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    // Because the tests module is an inner module,
    // we need to bring the code under test into the scope of the inner module.
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting_v1("Carol");
        assert!(result.contains("Carol"));

        let result = greeting_v2("Carol");
        assert!(
            result.contains("Carol"),
            // custom failure message
            "Greeting did not contain name, the actual greeting is `{result}`."
        );
    }
}
