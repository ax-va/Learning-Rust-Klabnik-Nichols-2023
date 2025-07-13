/*!
# My Crate

`my_crate` is a collection of utilities to make performing certain calculations more convenient.
*/

/**
Adds one to the number given.

# Examples

```
let arg = 5;
let answer = my_crate::add_one(arg);

assert_eq!(6, answer);
```
*/
pub fn add_one(x: i32) -> i32 {
    x + 1
}
/*
- Line-level documentation comments `//!` or block-style ones `/*! */`,
placed at the beginning of a module or crate,
document the enclosing item (like a module or crate).

- Line-level documentation comments `///` or
block-style ones `/** */` before items like functions, structs, etc.
are used to generate HTML documentation for them.
This command runs the `rustdoc` tool distributed with Rust and
puts the generated HTML documentation in the `target/doc` directory
```
$ cargo doc
```

Build the HTML for your current crate's documentation
(as well as the documentation for all of your crate's dependencies) and
open the result in a web browser
```
$ cargo doc --open
```

Run tests together with doc-tests
```
$ cargo test
...
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```

Common sections in documentations:
# Examples
# Panics
# Errors
# Safety
 */