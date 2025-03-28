/*

In programming languages with null, variables can always be in one of two states: null or not-null.
Rust doesn't have the null value.
The concept of null in Rust is not embedded in the language construct, but in its implementation using enum.

The `Option<T>` type is an enum defined by the standard library.
`<T>` is a generic type parameter that can be any type.
It encodes the very common scenario, in which a value could be either something or nothing.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Because `Option<T>` and `T` are different types,
the compiler won't let us use an `Option<T>` value as if it was definitely a valid value,
while a value of `T` is *always valid* and no check for null is needed in this case.

The `Option<T>` enum is so useful that it's even *included in the prelude*.
That means, there is no need to bring it into scope explicitly.

´Some(T)` and `None` are variants of type `Option<T>`.
`Some` and `None` can be directly used without the `Option::` prefix.

History notice

Tony Hoare, "Null References: The Billion Dollar Mistake", 2009:
"I call it my billion-dollar mistake.
At that time, I was designing the first comprehensive type system for references in an object-oriented language.
My goal was to ensure that all use of references should be absolutely safe,
with checking performed automatically by the compiler.
But I couldn't resist the temptation to put in a null reference, simply because it was so easy to implement.
This has led to innumerable errors, vulnerabilities, and system crashes,
which have probably caused a billion dollars of pain and damage in the last forty years."

```
$ cd 06*
$ cd 06-2*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() {
    let some_number = Some(5); // type: `Option<i32>`
    let some_char = Some('e'); // type: `Option<char>`
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // compilation error: "error[E0277]: cannot add `Option<i8>` to `i8"
    // let sum = x + y;
    //             ^ no implementation for `i8 + Option<i8>`
}
