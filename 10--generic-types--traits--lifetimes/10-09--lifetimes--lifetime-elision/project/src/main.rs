/*
```
$ cd 10*
$ cd 10-09*
$ cargo new project
$ cd project
$ cargo run
```

Three *lifetime elision rules*:

1. The compiler assigns an input lifetime parameter to each parameter that is a reference, e.g.,

    | before                     | after                                    |
    |----------------------------|------------------------------------------|
    | `fn foo(x: &i32)`          | `fn foo<'a>(x: &'a i32)`                 |
    | `fn foo(x: &i32, y: &i32)` | `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)` |

    and so on.


2. If there is exactly one input lifetime parameter,
that lifetime is assigned to all output lifetime parameters, e.g.,

    | before                           | after                               |
    |----------------------------------|-------------------------------------|
    | `fn foo<'a>(x: &'a i32) -> &i32` | `fn foo<'a>(x: &'a i32) -> &'a i32` |


3. If there are multiple input lifetime parameters,
but one of them is `&self` or `&mut self` because this is a method,
the lifetime of `self` is assigned to all output lifetime parameters.

 */

// In earlier versions of Rust, before the compiler included *lifetime elision rules*,
// the function signature would have been written like this:
// `fn get_first_line<'a>(s: &'a str) -> &'a str {`
fn get_first_line(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            return &s[0..i];
        }
    }

    return &s[..]
}

fn main() {
    let text= String::from("Hello, world!\nI am here!");
    let first_line = get_first_line(&text);
    println!("The first line: {first_line}");
    // The first line: Hello, world!
}
