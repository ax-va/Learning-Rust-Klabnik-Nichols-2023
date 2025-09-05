/*
Use case: recursive types

A value of a *recursive type* can have another value of the same type as part of itself.

Example: cons list, e.g., `(1, (2, (3, Nil)))`.

Each item in a cons list contains two elements:
the value of the current item and the next item.
 */

// compilation error: "error[E0072]: recursive type `List` has infinite size"
/*
enum List {
    Cons(i32, List),
    Nil,
}
 */

// Instead of storing a value directly,
// we should change the data structure to store the value indirectly
// by storing a pointer to the value instead.
enum List {
    Cons(i32, Box<List>),
    Nil,
}
// Because a `Box<T>` is a pointer,
// Rust always knows how much space a `Box<T>` needs:
// a pointer’s size doesn't change based on the amount of data it's pointing to.
// The `Box<List>` will point to the next `List` value
// that will be on the heap rather than inside the `Cons` variant.
// When a `Box<T>` value goes out of scope,
// the heap data that the box is pointing to is cleaned up.

use crate::List::{Cons, Nil};

fn main() {
    /*
    let list = Cons(1, Cons(2, Cons(3, Nil)));
    */
    let list = Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))
        ))
    ));
}
