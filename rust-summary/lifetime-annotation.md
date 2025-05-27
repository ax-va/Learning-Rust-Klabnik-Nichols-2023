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
    fn foo<'a>(x: &'a i32) -> &'a i32 { // ...
    ```

3. If there are multiple input lifetime parameters,
but one of them is `&self` or `&mut self` because this is a method,
the lifetime of self is assigned to all output lifetime parameters.
