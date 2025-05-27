/*
```
$ cd 10*
$ cd 10-06*
$ cargo new project --lib
$ cd project
$ cargo build
```
 */
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The maximum member is x = {}", self.x);
        } else {
            println!("The minimum member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a `trait for` any type that implements another trait.
// This is called *blanket implementations*.
// For example, the standard library implement the `ToString` trait on any `T` type that implements the `Display` trait.
/*
impl<T: Display> ToString for T {
    // ...
}
 */

// Integers implement the `Display` trait
// so we can the `to_string` method defined by the `ToString` trait.
/*
let s = 3.to_string();
 */
