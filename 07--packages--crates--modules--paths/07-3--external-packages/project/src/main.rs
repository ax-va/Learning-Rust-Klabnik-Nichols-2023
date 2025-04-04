/*
```
$ cd 07*
$ cd 07-3*
$ cargo new project
$ cd project
$ cargo run
```
 */

// Use external package to our project from the standard library included in Rust
use std::collections::HashMap;
// Use third-party package from https://crates.io
use rand::Rng;
// The `thread_rng()` function returns a `ThreadRng` instance, which implements the `Rng` trait.
// Rust requires that the trait be in scope to access its methods on the implementing type.

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}
