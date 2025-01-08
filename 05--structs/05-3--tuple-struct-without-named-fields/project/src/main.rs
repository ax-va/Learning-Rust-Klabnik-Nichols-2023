/*
```
$ cd 05*
$ cd 05-3*
$ cargo new project
$ cd project
```
 */

// two tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let color = Color(0, 128, 255);
    let point = Point(1, 2, 3);

    println!("{0}", color.1);
    // 128
    println!("{0}", point.2);
    // 3
}
