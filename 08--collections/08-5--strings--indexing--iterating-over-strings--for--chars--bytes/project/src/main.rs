/*
```
$ cd 08*
$ cd 08-5*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() {
    let s1 = String::from("hello");

    // The integer indexing of strings is invalid,
    // although `String` is a wrapper over `Vec<u8>`.

    // compilation error: "error[E0277]: the type `str` cannot be indexed by `{integer}`"
    // let h = s1[0];
    //            ^ string indices are ranges of `usize`

    let hello = String::from("Hola");
    let length = hello.len();
    println!("The length of \"{hello}\" is {length} bytes.");
    // The length of "Hola" is 4 bytes.
    // This means 1 byte is used for the UTF-8 encoding of 1 character.

    let hello = String::from("Привет");
    let length = hello.len();
    println!("The length of \"{hello}\" is {length} bytes.");
    // The length of "Привет" is 12 bytes.
    // This means 2 bytes are used for the UTF-8 encoding of 1 character.

    let hello = String::from("नमस्ते");
    let length = hello.len();
    println!("The length of \"{hello}\" is {length} bytes.");
    // The length of "नमस्ते" is 18 bytes.
    // No letters, but grapheme clusters.

    // Extract bytes by using string slices
    let hello = String::from("Привет");
    let s = &hello[0..4];
    println!("First four bytes of \"{hello}\" encoded as letters is \"{s}\".");
    // First four bytes of "Привет" encoded as letters is "Пр".
    let s = &hello.as_bytes()[0..4];
    println!("First four bytes: {:?}", s); // `{:?}` for debug formatting
    // First four bytes: [208, 159, 209, 128]

    // Iterate over string characters and bytes

    for chr in "Привет".chars() {
        println!("{chr}")
    }
    // П
    // р
    // и
    // в
    // е
    // т

    let hello = String::from("Hola");
    for chr in hello.chars() {
        println!("{chr}")
    }
    // H
    // o
    // l
    // a

    for byte in "Hola".bytes() {
        println!("{byte}");
    }
    // 72
    // 111
    // 108
    // 97
}
