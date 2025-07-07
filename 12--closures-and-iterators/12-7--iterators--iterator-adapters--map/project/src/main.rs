/*
```
$ cd 12*
$ cd 12-7*
$ cargo new project
$ cd project
$ cargo run
```

*Iterator adapters*
transform or extend an existing iterator to produce a new iterator.
They are *lazy*, meaning they don't do anything until the iterator is actually consumed.

Iterator adaptors:
- Take a reference (often `&mut self`) to the iterator;
- Return a new iterator, usually lazy (it doesn't run `next` until we iterate it).
- Can call `next` internally, but they don't consume the whole iterator themselves.
- Can take closures as arguments that capture their environment.
- Examples: `map`, `filter`, `peekable`, `enumerate`.
 */

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // Create a new iterator
    v1.iter().map(|x| x + 1); // lazy
    // Consume the iterator and collect the resultant values into a collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {v2:?}");
    // v2: [2, 3, 4]
    println!("v2: {v2:#?}"); // pretty print
    // v2: [
    //     2,
    //     3,
    //     4,
    // ]
}
