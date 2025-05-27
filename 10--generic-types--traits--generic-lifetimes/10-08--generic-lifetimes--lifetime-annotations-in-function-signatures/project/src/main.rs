/*
```
$ cd 10*
$ cd 10-08*
$ cargo new project
$ cd project
$ cargo run
```
 */

// The function should return the longer of two string slices
/*
fn longest_v1(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
 */

// The function does not compile because the Rust compiler
// cannot infer the lifetime of the returned reference.
// Rust requires explicit *lifetime annotations*
// when a function returns a reference that depends on input references.
// Without lifetimes, the compiler doesn't know
// whether the returned `&str` comes from `s1` or `s2`,
// or whether it's valid beyond the function's execution.

// compilation error
/*
error[E0106]: missing lifetime specifier
  --> src/main.rs:13:39
   |
13 | fn longest_v1(s1: &str, s2: &str) -> &str {
   |                   ----      ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`
help: consider introducing a named lifetime parameter
   |
13 | fn longest_v1<'a>(s1: &'a str, s2: &'a str) -> &'a str {
   |              ++++      ++           ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `project` (bin "project") due to 1 previous error
 */

// Declare the generic *lifetime* parameters just as we did with generic *type* parameters.
// The signature expresses the following constraint:
// the returned reference will be valid as long as both the parameters are valid.
fn longest_v2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
// Explanation:
// The function signature now tells Rust
// that for some lifetime `'a`, the function takes two parameters,
// both of which are string slices that live at least as long as lifetime `'a`.
// The function signature also tells Rust
// that the string slice returned from the function will live at least as long as lifetime `'a`.
// The *borrow checker* should reject any values that does not adhere to these constraints.
// The generic lifetime `'a` will get the concrete lifetime
// that is equal to the smaller of the lifetimes of `s1` and `s2`.
// The returned reference with the same lifetime parameter `'a`
// will also be valid for the length of the smaller of the lifetimes of `s1` and `s2`.

fn main() {

    // Leads to compilation error

    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_v2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
     */

   /*
    error[E0597]: `string2` does not live long enough
      --> src/main.rs:76:47
       |
    75 |         let string2 = String::from("xyz");
       |             ------- binding `string2` declared here
    76 |         result = longest_v2(string1.as_str(), string2.as_str());
       |                                               ^^^^^^^ borrowed value does not live long enough
    77 |     }
       |     - `string2` dropped here while still borrowed
    78 |     println!("The longest string is {result}");
       |                                     -------- borrow later used here

    For more information about this error, try `rustc --explain E0597`.
    error: could not compile `project` (bin "project") due to 1 previous error
     */

    let string1 = String::from("abcd");
    let string2 = "xyz";
    // let result = longest_v1(string1.as_str(), string2);
    let result = longest_v2(string1.as_str(), string2);
    println!("longest: {result}");
    // longest: abcd

    let string1 = String::from("long string is long"); // valid until the end of the outer scope

    {
        let string2 = String::from("xyz"); // valid until the end of the inner scope
        let result = longest_v2(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
        // The longest string is long string is long
    }
}
