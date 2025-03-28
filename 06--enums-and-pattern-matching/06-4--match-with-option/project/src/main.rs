/*
```
$ cd 06*
$ cd 06-4*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn plus_one_v1(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// The arms' patterns must cover all possibilities,
// i.e. they must be exhaustive:

// compilation error: error[E0004]: non-exhaustive patterns: `None` not covered
// fn plus_one_v2(x: Option<i32>) -> Option<i32> {
//     match x {
//           ^ pattern `None` not covered
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {
    let five = Some(5);
    let six = plus_one_v1(five); // `Some(6)`
    let none = plus_one_v1(None); // `None`
}
