/*
A *dangling pointer* is a pointer that references a location in memory that may have been given to someone else.
The memory can be freed up while preserving a pointer to that memory.
In Rust, the compiler guarantees that references will never be dangling references.

```
$ cd 04*
$ cd 04-6*
$ cargo new project
$ cd project
```
 */

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // Ownership is moved out, and nothing is deallocated.

// compilation error: "error[E0106]: missing lifetime specifier"
fn dangle() -> &String { // `dangle` returns a reference to a `String`.
//             ^ expected named lifetime parameter
    let s = String::from("hello"); // `s` is a new `String`.
    // compilation error: "error[E0515]: cannot return reference to local variable `s`"
    &s // // Return a reference to the String, `s`.
//  ^^ returns a reference to data owned by the current function
} // Here, `s` goes out of scope and is dropped, so its memory goes away.
// The reference would be pointing to an invalid `String`.

fn main() {
    let no_problem = no_dangle();
    let reference_to_nothing = dangle();
}
