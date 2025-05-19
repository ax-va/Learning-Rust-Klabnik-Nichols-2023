# Traits

*Traits* are similar to what many other languages call *interfaces*, 
and they define shared functionality that types can implement.

Each type implementing a trait must provide its own implementation of trait's method signatures.

```rust
pub trait MyTrait {
    fn some_trait_method(&self) -> SomeType;
}

pub struct MyType {
    // ...
}

impl MyTrait for MyType {
    fn some_trait_method(&self) -> SomeType {
        // Implement the method
        // ...
    }
}
```

## Coherence

We can implement a trait on a type in a crate *only* 
if either the trait or the type, or both, are local to the crate.
But we cannot implement external traits on external types.
This restriction is part of a property called *coherence*, and more specifically the *orphan rule*.
This rule ensures that other people's code cannot break your code and vice versa.

## Default Implementations

We can also define a default implementation in a trait.
Default implementations can call other methods in the same trait, 
even if those other methods don't have a default implementation.
We cannot call the default implementation from an overriding implementation of that same method.

```rust
pub trait MyTrait {    
    // default implementation
    fn some_trait_method(&self) -> SomeType {
        // Implement the method
        // ...
    }
}

pub struct MyType {
    // ...
}

impl MyTrait for MyType { 
    // Use the default implementation
}
```

## Traits as parameters and trait bounds

We can use traits as parameters.

This `impl` is actually *syntax sugar*
```rust
pub fn some_function(item: &impl MyTrait) {
    println!("Using a trait method! {}", item.some_trait_method());
}
```

for a longer form known as a *trait bound*
```rust
pub fn some_function<T: MyTrait>(item: &T) {
    println!("Using a trait method! {}", item.some_trait_method());
}
```

Similarly, for two parameters
```rust
pub fn some_function(item1: &impl MyTrait, item2: &impl MyTrait) {
    // ...
}
```

or on generic types
```rust
pub fn some_function<T: MyTrait>(item1: &T, item2: &T) {
    // ...
}
```

Specify multiple trait bounds with the `+` syntax to use methods from multiple traits in the function
```rust
pub fn some_function(item: &(impl MyTrait + Display)) {
    // ...
}
```

or on generic types
```rust
pub fn some_function<T: Summary + Display>(item: &T) {
    // ...
}
```