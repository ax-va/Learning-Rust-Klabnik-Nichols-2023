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
- `-> &'a str` means the returned reference is valid for at least `'a`;

