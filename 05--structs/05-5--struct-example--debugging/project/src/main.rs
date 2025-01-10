/*
This example demonstrates calculating the area
of a rectangle *without* the use of structs.

```
$ cd 05*
$ cd 05-5*
$ cargo new project
$ cd project
```
 */

// Problem:
// it's not clear anywhere in the program
// that the parameters are related.
fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

// Problem:
// tuples don't name their elements,
// so we have to index into the parts of the tuple,
// making our calculation less obvious.
// Moreover, if we want to draw the rectangle on the screen,
// mixing up the width and height would matter.
fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


// Add the trait `Debug` to the struct
// to print out debugging information.
#[derive(Debug)]
struct Rectangle {
    // The width and height are related to each other,
    // and it gives descriptive names to the values.
    width: u32,
    height: u32,
}

fn area_v3(rectangle: &Rectangle /* immutable borrow type*/) -> u32 {
    // Accessing fields of a borrowed struct instance
    // *does not move* the field values.
    rectangle.width * rectangle.height
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {0} square pixels.",
        area_v1(width, height)
    );
    // The area of the rectangle is 1500 square pixels.

    let rect = (30, 50);

    println!(
        "The area of the rectangle is {0} square pixels.",
        area_v2(rect)
    );
    // The area of the rectangle is 1500 square pixels.

    let rect2 = Rectangle {
        // Define fields
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {0} square pixels.",
        area_v3(&rect2)
    );

    // print by using `#[derive(Debug)]`
    println!("`rect2` is {:?}", rect2);
    // `rect2` is Rectangle { width: 30, height: 50 }

    // pretty print by using `#[derive(Debug)]`
    println!("`rect2` is {:#?}", rect2);
    // `rect2` is Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    // Another way to print out a value using the `Debug` format is
    // to use the `dbg!` macro, which takes ownership of an expression
    // (as opposed to `println!`, which takes a reference to `stdout`),
    // prints the file and line number of where that `dbg!` macro call occurs
    // in the code along with the resultant value of that expression to `stderr`, and
    // returns ownership of the value.

    let scale = 2;
    let rect3 = Rectangle {
        // Because `dbg!` returns ownership of the expression's value,
        // the `width` field will get the same value as if we didn't have the `dbg!` call.
        width: dbg!(30 * scale),
        height: 50,
    };
    // We don't want dbg! to take ownership of `rect3`,
    // so we use a reference to `rect3`.
    dbg!(&rect3);
    // [src/main.rs:97:16] 30 * scale = 60
    // [src/main.rs:102:5] &rect3 = Rectangle {
    //     width: 60,
    //     height: 50,
    // }

    // This output uses the pretty `Debug` formatting of the `Rectangle` type.
}
