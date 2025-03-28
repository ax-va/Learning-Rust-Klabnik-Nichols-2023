/*
Notices:

1.

All functions defined within an `impl` block are called `associated functions`
that are associated with their types, for example, methods, constructors, and so on.

2.

Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and
their first parameter is always `self`, which represents the instance of the struct the method is being called on.

3.

In Rust, the following are the same:
```
p1.distance(&p2);
(&p1).distance(&p2);
```

4.

Associated functions that aren't methods are often used for constructors
that will return a new instance of the struct.
These are often called `new`, but `new` isn't a special name and isn't built into the language.
To call a associated function that is no method, the `::` syntax with the struct name is used.
The `::` syntax is used for both associated functions and namespaces created by modules.

5.

Each struct is allowed to have multiple `impl` blocks.

```
$ cd 05*
$ cd 05-6*
$ cargo new project
$ cd project
$ cargo run
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
    // associated functions that are methods
    fn area(&self /* short for `self: &Self` */) -> u32 {
        // Within an `impl` block, the type `Self` is
        // an alias for the type that the `impl` block is for.
        // Methods can take ownership of `self`, borrow `self` immutably,
        // as here, or borrow `self` mutably.
        // If we need to change the instance, we should use `&mut self`.
        self.width * self.height
    }

    // A field and an associated function can have the same name
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function as constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// using multiple `impl` blocks
impl Rectangle {
    fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
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
    println!("Can `rect1` hold `rect2`? {}", rect1.can_hold(&rect2));
    // Can `rect1` hold `rect2`? true
    println!("Can `rect1` hold `rect3`? {}", rect1.can_hold(&rect3));
    // Can `rect1` hold `rect3`? false

    // Call the associated function as constructor
    let sq = Rectangle::square(3);

    // using a method from the second `impl` block
    println!("Is `sq` valid? {}", sq.is_valid());
    // Is `sq` valid? true
}
