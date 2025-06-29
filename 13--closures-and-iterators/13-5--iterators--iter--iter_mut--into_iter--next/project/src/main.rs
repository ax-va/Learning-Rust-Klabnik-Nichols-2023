/*
```
$ cd 13*
$ cd 13-5*
$ cargo new project
$ cd project
$ cargo run
```

All iterators implement the `Iterator` trait that is defined in the standard library.

Methods:

- `iter()`
returns an iterator that borrows each item immutably and
does not take ownership of the collection;

- `iter_mut()`
returns an iterator that borrows each item mutably and
does not take ownership, but needs a mutable reference to the collection;

- `into_iter()`
returns an iterator that takes ownership of the items.
The collection is moved and cannot be used after.

 */

fn main() {

    // ------ //
    // `iter` //
    // ------ //

    // In Rust, iterators are *lazy*.
    // This means they have no effect
    // until we call methods that consume the iterator to use it up.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for x in v1_iter {
        println!("Got: {x}"); // x is of type `&i32`
    } // `v1` is still usable, but `v1_iter` is not, because it was moved into the `for` loop with ownership
    // Got: 1
    // Got: 2
    // Got: 3

    // compilation error: "error[E0382]: use of moved value: `v1_iter`"
    // for x in v1_iter {
    //          ^^^^^^^ value used here after move
    //     println!("Got: {x}");
    // }

    for x in v1.iter() {
        println!("Got: {x}");
    }
    // Got: 1
    // Got: 2
    // Got: 3

    // ---------- //
    // `iter_mut` //
    // ---------- //

    let mut v2 = vec![1, 2, 3];
    for x in v2.iter_mut() {
        *x += 1; // x is of type `&mut i32`
    }
    println!("v2: {v2:?}");
    // v2: [2, 3, 4]

    // ----------- //
    // `into_iter` //
    // ----------- //

    let v3 = vec![1, 2, 3];
    for x in v3.into_iter() {
        println!("Got: {}", x); // x is of type `i32`
    } // `v3` is no longer usable
    // Got: 1
    // Got: 2
    // Got: 3

    // compilation error: "error[E0382]: borrow of moved value: `v3`"
    // println!("v3: {:?}", v3);
    //                      ^^ value borrowed here after move
}
