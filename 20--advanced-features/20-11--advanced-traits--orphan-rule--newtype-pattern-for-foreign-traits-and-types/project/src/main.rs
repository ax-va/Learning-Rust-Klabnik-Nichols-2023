/*
The *orphan rule* says:
- You can implement a trait for a type only
if either the trait or the type is defined in your crate.

So we cannot implement a foreign trait for a foreign type.

The workaround is using the *newtype pattern*,
which involves creating a new type in a tuple struct.

Implementing `Deref` (and `DerefMut` when needed) makes
the wrapper transparently forward many method calls to the inner type.
In order to restrict the wrapper's interface,
avoid `Deref` and manually expose only the methods you choose.
 */

use std::fmt; // the `fmt` format module within the `std` crate

// not allowed
// impl Display for Vec<i32> {}

// new type as a thin wrapper
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use `self.0` to access the inner `Vec<T>`
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(
        vec![
            String::from("hello"),
            String::from("world"),
        ]
    );
    println!("w = {w}");
    // w = [hello, world]
}
