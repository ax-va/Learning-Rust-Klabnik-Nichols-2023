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

Rust ensures *at compile time* that method calls are valid, including trait implementations.

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

## Traits as Parameters and Trait Bounds

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

### See also

- https://doc.rust-lang.org/reference/trait-bounds.html

### The `where` Clause

Instead of

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
}
```

use a `where` clause for clearer trait bounds

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## Return Types that Implement Traits

```rust
fn return_my_traitable() -> impl MyTrait {
    // Return a type that implements `MyTrait`
}
```

However, we can *only* use `impl Trait` if we are returning a *single type*.
For example, we *cannot* return in that function optionally either `MyType1` or `MyType2`, 
even if the both types implement `MyTrait`.

## Conditional Implementations and Blanket Implementations

- A *conditional implementation* for traits means
that a trait is implemented for a type *only if* certain conditions or constraints are met.

- A *blanket implementation* is when a trait is implemented for all types that satisfy certain trait bounds.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// unconditional implementation
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// conditional implementation
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The maximum member is x = {}", self.x);
        } else {
            println!("The minimum member is y = {}", self.y);
        }
    }
}
```

### Blanket Implementation

```rust
// This says: "Implement the `ToString` trait for any type `T`, but only if `T` also implements `Display`".
// This means every type that implements `Display` automatically gets a `ToString` implementation.
impl<T: Display> ToString for T {
    // ...
}
```

Integers implement the `Display` trait,
so we can the `to_string` method defined by the `ToString` trait.
```rust
let s = 3.to_string();
```
