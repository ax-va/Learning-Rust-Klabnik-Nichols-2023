/*
```
$ cd 04*
$ cd 04-7*
$ cargo new project
$ cd project
$ cargo run
```

String slices:

- A string slice is essentially a reference that points to a specific range within a string.

- Internally, a string slice consists of a pointer to the starting byte of the string data
and a length indicating how many bytes are in the slice.

- String slices are immutable, meaning you cannot modify the contents they point to through the slice.
This immutability ensures safety and prevents data races.

- Since slices are references, they do not own the data they point to.
Therefore, the original string must remain valid while the slice is in use to avoid dangling references.

Difference between string slices and strings:

- The `&str` type is an immutable reference to a string slice that does not own the data.
- The `String` type is an owned, heap-allocated string that is mutable and growable.

 */

fn first_word(s: &str) -> &str { // The `&str` type is an immutable reference to a string slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // or `&s`
}

fn main() {
    let s = String::from("hello world");
    // string slices
    let hello = &s[..5];
    let world = &s[6..];

    println!("{hello}");
    // hello
    println!("{world}");
    // world

    let s = String::from("hallo welt");

    println!("{hello}");
    // hello
    println!("{world}");
    // world

    let hello = &s[..5];
    let world = &s[6..];

    println!("{hello}");
    // hallo
    println!("{world}");
    // welt

    // Notice:
    // String slice range indices must occur at valid UTF-8 character boundaries.
    // If you attempt to create a string slice in the middle of a multibyte character,
    // your program will exit with an error.

    let first = first_word(&s);
    println!("{first}");
    // hallo

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // compilation error: "error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable"
    // s.clear(); // error!
    // ^^^^^^^^^ mutable borrow occurs here
    println!("the first word is: {word}");
    // the first word is: hello

    s.clear();
    println!("cleared: {s}.");
    // cleared: .

    // *String literals* in Rust, such as "Hello, world!", are of type `&'static str`.
    // They are string slices that have a `'static` lifetime,
    // meaning they are valid for the entire duration of the program.
    // This is because string literals are stored directly in the program's binary.
    let s = "Hello, world!"; // Type of `s` is `&'static str`

    let my_string = String::from("hello world"); // string
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world"; // string literal
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already, this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // This slice has the type `&[i32]`
    assert_eq!(slice, &[2, 3]);
}
