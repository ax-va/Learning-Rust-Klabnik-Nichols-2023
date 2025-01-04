/*

The action of creating a reference is called *borrowing*.

```
$ cd 04*
$ cd 04-4*
$ cargo new project
$ cd project
```
 */

fn main() {
    let s1 = String::from("hello");
    // Use a function that uses a *reference* to an object as a parameter
    // instead of taking ownership of the value.
    let len = calculate_length(&s1); // Ampersands represent references.
    // `&s1` refers to the value of `s1` but does not own it.
    // The value it points to will not be dropped when the reference stops being used.
    println!("The length of '{s1}' is {len}.");
    // The length of 'hello' is 5.

    let s = String::from("hello");
    // If variables are immutable, references are immutable too.
    change(&s);
}

fn calculate_length(s: &String) -> usize { // The type of the parameter `s` is a reference.
    s.len() // Don't need to return the value in order to give back ownership.
} // Here, `s` goes out of scope.
// But because it does not have ownership of what it refers to, the `String` is not dropped.

fn change(some_string: &String) {
    // compilation error: "error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference"
    // some_string.push_str(", world");
    // ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}