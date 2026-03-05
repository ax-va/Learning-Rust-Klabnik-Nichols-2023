/*
*Dynamically sized types* (DSTs) are types whose size is not known at compile time
and must therefore be used behind a pointer that carries size information at runtime.

Rust has two major DST categories:

1. Sequence-like DSTs
- `str`
- `[T]`
-> metadata = length

2. Trait object DSTs
- `dyn Trait`
-> metadata = vtable

Both require pointers to be usable.
 */

fn main() {
    // 1. `str` with different pointers

    // `str` is a dynamically sized type (DST) and represents a sequence of UTF-8 bytes.
    // The length of that sequence:
    // - is not known at compile time;
    // - varies at runtime.
    // So the compiler cannot know how large a plain str value would be.

    // compilation error: "error[E0277]: the size for values of type `str` cannot be known at compilation time"
    // let s: str = "hello";

    let s: &str = "hello";

    // A reference to a sized type is a *thin pointer* that stores only the address,
    // while a `&str` is a *fat pointer* that stores both the address of the data and its length:
    // - a pointer to the first byte,
    // - the length of the string.

    // Even though `str` itself is a dynamically sized type (DST),
    // the compiler does know the fixed size of `&str` at compile time,

    use std::rc::Rc;
    // Also fine
    let s: &str;
    let s: Box<str>;
    let s: Rc<str>;

    // 2. Trait objects are also dynamically sized types

    trait Draw {
        fn draw(&self);
    }

    // compilation error: "error[E0782]: expected a type, found a trait"
    // let x: Draw;

    // `dyn Draw` is a trait object and a dynamically sized type.
    // Trait objects must be behind a pointer.
    let x: &dyn Draw;
    let x: Box<dyn Draw>;
    let x: Rc<dyn Draw>;
}