/*
Three *lifetime elision rules*:

1. The compiler assigns a lifetime parameter to each parameter that is a reference,
e.g.
```
fn foo<'a>(x: &'a i32) // ...

fn foo<'a, 'b>(x: &'a i32, y: &'b i32) // ...

// and so on.
```

2. If there is exactly one input lifetime parameter,
that lifetime is assigned to all output lifetime parameters:
```
fn foo<'a>(x: &'a i32) -> &'a i32 // ...
```

3. If there are multiple input lifetime parameters,
but one of them is `&self` or `&mut self` because this is a method,
the lifetime of self is assigned to all output lifetime parameters.

```
$ cd 10*
$ cd 10-10*
$ cargo new project
$ cd project
$ cargo run
```
 */

// In earlier versions of Rust, before the compiler included *lifetime elision rules*,
// the function signature would have been written like this.
// `fn first_word<'a>(s: &'a str) -> &'a str {`
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}

fn main() {
    println!("Hello, world!");
}
