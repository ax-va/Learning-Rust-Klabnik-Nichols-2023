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

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {0} square pixels.",
        area_v1(width, height)
    );
    // The area of the rectangle is 1500 square pixels.

    let rectangle = (30, 50);

    println!(
        "The area of the rectangle is {0} square pixels.",
        area_v2(rectangle)
    );
    // The area of the rectangle is 1500 square pixels.
}
