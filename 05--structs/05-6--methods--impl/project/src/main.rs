/*
Notice 1:

Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and
their first parameter is always `self`, which represents the instance of the struct the method is being called on.

Notice 2:

In Rust, the following are the same:
```
p1.distance(&p2);
(&p1).distance(&p2);
```

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
        // If we need to change the instance, we should use `&mut self`.
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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

    // method
    if rect.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            // struct's field
            rect.width
        );
        // The rectangle has a nonzero width; it is 30
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // Can rect1 hold rect2? true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // Can rect1 hold rect3? false
}
