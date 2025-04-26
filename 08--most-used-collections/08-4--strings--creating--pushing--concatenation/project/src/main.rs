/*
```
$ cd 08*
$ cd 08-4*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() {
    // Create a mutable, empty string instance
    let mut s = String::new();

    // Create a string from a string literal:

    // 1. by using the `to_string` method from the `Display` trait.
    let data = "initial contents";
    let s = data.to_string();
    // or
    let s = "initial contents".to_string();

    // 2. by using the `String::from` method
    let s = String::from("initial contents");

    // UTF-8 encoding
    let hello = String::from("Hallo!");
    let hello = String::from("こんにちは!");
    let hello = String::from("안녕하세요!");
    let hello = String::from("你好!");
    let hello = String::from("Привет!");
    let hello = String::from("Hola!");

    // Append a string literal
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");
    // s is foobar

    // Explanation:
    // - "bar" is borrowed as a string slice argument to `push_str`;
    // - once the method finishes, the borrow ends;
    // - `push_str` copies, not moves or borrows long-term;
    // - the `String` and `&'static str` instances own their data.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    // s1 is foobar
    println!("s2 is {s2}");
    // s2 is bar

    // Append a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");
    // s is lol

    // concatenation with the `+` operator and the `format!` macro //

    // Concatenate two strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // `s1` has been moved here into the add call and can no longer be used.
    // This statement actually takes ownership of `s1`,
    // appends a copy of the contents of `s2`,
    // and then returns ownership of the result.
    println!("s3 is {s3}");
    // s3 is Hello, world!

    // Concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");
    // s is tic-tac-toe

    // Use the `format!` macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    // This call doesn’t take ownership of any of its parameters
    println!("s is {s}");
    // s is tic-tac-toe
}
