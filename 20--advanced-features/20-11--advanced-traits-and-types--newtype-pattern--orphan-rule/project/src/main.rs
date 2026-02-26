/*
1. The *newtype pattern* for foreign traits and types

The *orphan rule* says:
- You can implement a trait for a type only
if either the trait or the type is defined in your crate.

So we cannot implement a foreign trait for a foreign type.

The workaround is using the newtype pattern,
which involves creating a new type in a tuple struct.

Implementing `Deref` (and `DerefMut` when needed) makes
the wrapper transparently forward many method calls to the inner type.
In order to restrict the wrapper's interface,
avoid `Deref` and manually expose only the methods you choose.

2. The *newtype pattern* for type safety and abstraction

The newtype pattern wraps an existing type in a new, distinct type
to provide stronger type safety and prevent mixing values
that have the same underlying representation but different meanings.
It also enables abstraction by hiding the inner type and
exposing only the operations you choose.
Because the wrapper is a separate type, you can implement traits and
enforce constraints without changing the original type.
 */

// Example 1

use std::fmt; // the `fmt` format module within the `std` crate

// not allowed
// impl fmt::Display for Vec<i32> {}

// new type as a thin wrapper
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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


// Example 2

// Differentiate `u64` to `UserId` and `OrderId` instead of just using
// `fn load_user(user_id: u64) {}` and `fn load_order(order_id: u64) {}`.

struct UserId(u64);
struct OrderId(u64);

fn load_user(user_id: UserId) {}
fn load_order(order_id: OrderId) {}