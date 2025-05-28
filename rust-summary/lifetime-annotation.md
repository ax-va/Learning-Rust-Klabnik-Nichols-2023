# Lifetime Annotation

A *lifetime* represents the scope for which a reference is valid.
*Lifetime annotations* are a way to tell the compiler how long references should be valid. 
Rust uses lifetimes to prevent *dangling references* and ensure *memory safety* without a *garbage collector*.

### Lifetime Annotation Syntax

```rust
fn some_function<'a>(x: &'a str) -> &'a str {
    x
}
```

The signature expresses the following constraint:
*"The returned reference will be valid as long as the function parameter is valid"*.

Here
- `'a` is a lifetime parameter;
- `x: &'a str` means `x` is a reference to a `str` that lives at least as long as `'a`;
- `-> &'a str` means the returned reference is valid for at least `'a`.

### Lifetime Elision Rules

There are three *lifetime elision rules* the compiler includes, which are used to derive lifetimes:

1. The compiler assigns an input lifetime parameter to each parameter that is a reference,  e.g.,

    | before                     | after                                    |
    |----------------------------|------------------------------------------|
    | `fn foo(x: &i32)`          | `fn foo<'a>(x: &'a i32)`                 |
    | `fn foo(x: &i32, y: &i32)` | `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)` |
    and so on.


2. If there is exactly one input lifetime parameter,
that lifetime is assigned to all output lifetime parameters:

    | before                           | after                               |
    |----------------------------------|-------------------------------------|
    | `fn foo<'a>(x: &'a i32) -> &i32` | `fn foo<'a>(x: &'a i32) -> &'a i32` |


3. If there are multiple input lifetime parameters,
but one of them is `&self` or `&mut self` because this is a method,
the lifetime of `self` is assigned to all output lifetime parameters.

### Lifetime Annotations in Struct and Method Definitions

```rust
struct SomeStruct<'a> {
    // The struct has the single field part
    // that holds a string slice, which is a reference.
    some_field: &'a str,
}

// Lifetime names for struct fields always need to be
// 1. declared after the `impl` keyword
// 2. and then used after the struct's name.
impl<'a> ImportantExcerpt<'a> {
    // The first elision rule applies to each parameter that is a reference
    fn some_method(&self) -> i32 {
        3
    }

    // The third lifetime elision rule applies to the output lifetime parameter
    fn another_method(&self, param: &str) -> &str {
        println!("Parameter: {param}");
        self.some_field
    }
}
```

### The Static Lifetime

The static lifetime `'static` denotes that the affected reference can live for the entire duration of the program.
All string literals have the `'static` lifetime, 
because the text of string literals is stored directly in the program's binary, 
which is always available.

```rust
let s: &'static str = "I have a static lifetime.";
```
