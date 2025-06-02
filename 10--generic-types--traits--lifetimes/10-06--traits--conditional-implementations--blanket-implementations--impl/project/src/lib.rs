/*
```
$ cd 10*
$ cd 10-06*
$ cargo new project --lib
$ cd project
$ cargo build
```

Notices:

- A *conditional implementation* for traits means
that a trait is implemented for a type *only if* certain conditions or constraints are met.

- A *blanket implementation* is when a trait is implemented for all types that satisfy certain trait bounds.

 */
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// unconditional implementation
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// conditional implementation
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The maximum member is x = {}", self.x);
        } else {
            println!("The minimum member is y = {}", self.y);
        }
    }
}

/* blanket implementation */

/*
```
// This says: "Implement the `ToString` trait for any type `T`, but only if `T` also implements `Display`".
// This means every type that implements `Display` automatically gets a `ToString` implementation.
impl<T: Display> ToString for T {
    // ...
}
```
 */

// Integers implement the `Display` trait,
// so we can the `to_string` method defined by the `ToString` trait.
/*
```
let s = 3.to_string();
```
 */
