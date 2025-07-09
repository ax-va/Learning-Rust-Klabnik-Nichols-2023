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
Line-level documentation comments that begin with `///` or
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
 */