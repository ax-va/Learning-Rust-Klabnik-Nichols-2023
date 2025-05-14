# Traits

*Traits* are similar to what many other languages call *interfaces*, 
and they define shared functionality that types can implement.

Each type implementing a trait must provide its own implementation of trait's method signatures.

```rust
pub trait MyTrait {
    fn some_method(&self) -> SomeType;
}

pub struct MyType {
    // ...
}

impl MyTrait for MyType {
    fn some_method(&self) -> SomeType {
        // Implement the method
        // ...
    }
}
```

We can implement a trait on a type in a crate *only* if either the trait or the type, or both, are local to the crate.
But we cannot implement external traits on external types.
This restriction is part of a property called *coherence*, and more specifically the *orphan rule*.
This rule ensures that other people's code cannot break your code and vice versa.