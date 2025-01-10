/*
Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and
their first parameter is always `self`, which represents the instance of the struct the method is being called on.

```
$ cd 05*
$ cd 05-6*
$ cargo new project
$ cd project
```
 */

// Add the trait `Debug` to the struct
// to print out debugging information.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Everything within this `impl` block
// will be associated with the `Rectangle` type.
impl Rectangle {
    // method
    fn area(&self) -> u32 { // `&self` is actually short for `self: &Self`.
        // Within an `impl` block, the type `Self` is
        // an alias for the type that the `impl` block is for.
        // Methods can take ownership of `self`, borrow `self` immutably,
        // as here, or borrow `self` mutably.
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // method syntax
        rect.area()
    );
    // The area of the rectangle is 1500 square pixels.
}
