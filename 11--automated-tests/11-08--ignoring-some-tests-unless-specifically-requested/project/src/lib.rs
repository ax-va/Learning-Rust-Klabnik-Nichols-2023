/*
```
$ cd 11*
$ cd 11-08*
$ cargo new project --lib
$ cd project
```

- Run all the tests excluding ones marked with `ignore` attribute.
```
$ cargo test
...
running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
```

- Run only the ignored tests.
```
$ cargo test -- --ignored
...
running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
...
```

- Run all tests whether they are ignored or not.
```
$ cargo test -- --include-ignored
...
running 2 tests
test expensive_test ... ok
test it_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
```
 */

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}