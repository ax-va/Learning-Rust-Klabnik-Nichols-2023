/*
```
$ cd 08*
$ cd 08-6*
$ cargo new project
$ cd project
$ cargo run
```
 */
use std::collections::HashMap; // not included in the prelude automatically

fn main() {
    // Create an mutable, empty hash map
    let mut scores = HashMap::new();
    // Add key-value pairs
    // with keys of type `String` and values of type `i32`.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Access the values
    let team_name = String::from("Blue");
    let score = scores
        // The `get` method returns an `Option<&V>`
        .get(&team_name)
        // Call `copied` to get `Option<i32>` rather than `Option<&i32>`
        .copied()
        // Set the value to zero if the hash map does not have an entry for the key
        .unwrap_or(0);

    // Iterate over each key-value pair in an *arbitrary order*
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // Yellow: 50
    // Blue: 10

    // For types that implement the `Copy` trait, like `i32`,
    // the keyas and values are copied into the hash map.
    // For owned values like `String`, the values will be moved
    // and the hash map will be the owner of those values.

    let key = String::from("Favorite color");
    let value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(key, value);
    // `key` and `value` are invalid at this point

    // compilation error: "error[E0382]: borrow of moved value: `key`"
    // println!("{key}")
    //           ^^^^^ value borrowed here after move

    // compilation error: "error[E0382]: borrow of moved value: `value`"
    // println!("{value}")
    //           ^^^^^^^ value borrowed here after move

    // If we insert references to values into the hash map,
    // the values will not be moved into the hash map.
    // The values that the references point to must
    // be valid for at least as long as the hash map is valid.
}
