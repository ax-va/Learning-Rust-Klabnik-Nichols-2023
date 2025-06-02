/*
```
$ cd 10*
$ cd 10-11*
$ cargo new project
$ cd project
$ cargo run
```
 */

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    s1: &'a str,
    s2: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    // That is why the `Display` trait bound is necessary
    println!("Announcement! {ann}");
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("abcd"); // type: `String`, a heap-allocated, growable, and owned string type
    let string2 = "xyz"; // type: `&'static str`; instance: a string slice pointing to a string literal
    let ann = "All in one function";

    let result = longest_with_an_announcement(string1.as_str(), string2, ann);
    // Announcement! All in one function

    println!("Result: {result}");
    // Result: abcd

    // `ann` lives for the entire duration of the program
    println!("Announcement: {ann}");
    // Announcement: All in one function
}
