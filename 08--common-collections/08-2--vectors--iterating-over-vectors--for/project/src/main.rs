/*
```
$ cd 08*
$ cd 08-2*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() {
    // Create an immutable vector by using macro
    let vector = vec![100, 32, 57];

    // Iterate through immutable references to elements
    for value in &vector {
        println!("{value}");
    }
    // 100
    // 32
    // 57

    // Create a mutable vector by using macro
    let mut vector = vec![100, 32, 57];

    // Iterate through mutable references to elements and change them
    for value in &mut vector {
        // Use the `*` dereference operator to get to the value in `value`
        *value *= 10;
        println!("{value}");
    }
    // 1000
    // 320
    // 570
}
