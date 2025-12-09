struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    assert_eq!(3, feet);
    assert_eq!(10, inches);
    assert_eq!(3, x);
    assert_eq!(-10, y);
}
