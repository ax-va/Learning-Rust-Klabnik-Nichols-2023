/*
```
$ cd 08*
$ cd 08-7*
$ cargo new project
$ cd project
$ cargo run
```
 */
use std::collections::HashMap; // not included in the prelude automatically

fn main() {
    // Update a hash map with overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // {"Blue": 25}

    // Update a hash map with adding a key and value
    // only if a key is not present.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores
        // The `entry` method returns an enum called `Entry`
        // that represents a value that might or might not exist.
        .entry(String::from("Yellow"))
        // The `or_insert` method returns a mutable reference
        // to the value for the corresponding `Entry` key if that key exists,
        // and if not, it inserts the parameter as the new value for this key
        // and returns a mutable reference to the new value.
        .or_insert(50);
    // The value will be not overwritten
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    // {"Yellow": 50, "Blue": 10}

    // Update a value based on the old value.
    // Count how many times each word appears in some text.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // The `split_whitespace` method returns an iterator over subslices, separated by whitespace
    for word in text.split_whitespace() {
        // Return the mutable reference to the value
        let count = map.entry(word).or_insert(0);
        // The value is contained in and owned by the hash map,
        // but we can change it by dereferencing.
        *count += 1;
    } // The mutable reference goes out of scope
    println!("{:?}", map);
    // {"hello": 1, "world": 2, "wonderful": 1}

}
