/*
This code teaches how *operator overloading* works in Rust
using the `Add` trait, associated types, and default generic type parameters.
 */

// The `Add` trait defines how the `+` operator works
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
// The derive attributes automatically implement:
// - `Debug` -> allows printing
// - `Copy` -> values are copied instead of moved
/*
let p1 = Point { x: 1, y: 2 };
let p2 = p1; // copied
 */
// `Copy` isn't required by `Add`,
// but when a type implements `Copy`,
// using the `+` operator duplicates the operands
// instead of moving them, allowing them to be used afterward.
// - `Clone` -> can be duplicated
// `#[derive(Clone)]` auto-generates `impl Clone for Point`.
// So now we can do:
/*
let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();
 */
// `Clone` is included only because `Copy` requires `Clone`.
// Rust doesn't automatically derive dependent traits.
// - `PartialEq` -> allows `==` and `assert_eq!`
struct Point {
    x: i32,
    y: i32,
}

// We don't need `Add<Point>` here because of the default generic parameter
impl Add for Point {
    // The `Add` trait has an associated type named `Output`
    // that determines the type returned from the `add` method.
    type Output = Point;

    // Rust requires every parameter to have an explicit type.
    // The compiler won't infer the type of `other`
    // from the trait's default generic parameter.
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, scalar: i32) -> Point {
        Point {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

fn main() {
    // `a + b` is resolved to `Add::add(a, b)`
    assert_eq!(
        Point { x: 1, y: 2 } + Point { x: 2, y: 1 },
        Point { x: 3, y: 3 }
    );
    // `add(self, ...)` takes ownership of self.
    // Without `Copy`, using `a + b` would consume `a` and `b`
    // (move `a` and `b` into the `add` method) -> We could not use them anymore.
    // With `Copy`, it just duplicates it.

    assert_eq!(
        Point { x: 1, y: 2 } + 1,
        Point { x: 2, y: 3 }
    );
}

// - The default generic type in this code is within the `Add` trait.
// - `Rhs` means "right-hand side" in the default generic parameter `<Rhs=Self>`.
// - `self` in `add` means that the left operand should be moved into the function.
/*
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
 */