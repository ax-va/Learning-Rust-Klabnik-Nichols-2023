struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 0, y: 5 };

    // Create the variables `a` and `b`
    // that match the values of the `x` and `y`
    // fields of the `point` struct.
    // The names of the variables in the pattern
    // don't have to match the field names of the struct.
    let Point { x: a, y: b } = point;
    assert_eq!(0, a);
    assert_eq!(5, b);

    // This is a shorthand to create the variables `x` and `y`
    // that match the `x` and `y` fields of the `point` variable.
    let Point { x, y } = point;
    assert_eq!(0, x);
    assert_eq!(5, y);

    match point {
        // Stop checking arms once it has found the first matching pattern
        Point { x, y: 0 } => println!("On the x axis: ({x}, {y})"),
        Point { x: 0, y } => println!("On the y axis: ({x}, {y})"), // <-
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
    // On the y axis: (0, 5)
}
