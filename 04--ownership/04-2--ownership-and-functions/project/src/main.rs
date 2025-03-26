/*
Passing a variable to a function will move or copy, just as assignment does.

```
$ cd 04*
$ cd 04-2*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn takes_ownership(some_string: String) { // `some_string` comes into scope.
    println!("{some_string}");
} // Here, `some_string` goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // `some_integer` comes into scope.
    println!("{some_integer}");
} // Here, `some_integer` goes out of scope. Nothing special happens.

fn main() {
    let s = String::from("hello"); // `s` comes into scope.
    takes_ownership(s); // The value of `s` moves into the function.
    // `s` is no longer valid.

    let x = 5; // `x` comes into scope.
    makes_copy(x); // The copy of `x` goes into the function.
    // `x` is valid because but `i32` implements `Copy`.
} // Here, `x` goes out of scope, then `s`.
// However, because the value of `s` was moved, nothing special happens.
