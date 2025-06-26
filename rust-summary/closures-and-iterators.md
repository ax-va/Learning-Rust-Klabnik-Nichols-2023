# Functional Language Features

## Closures

In Rust, closures are anonymous functions that capture their environment.
We can save in closures a variable or pass them as arguments to other functions.

In Rust, closures usually don't require you to annotate 
the types of their parameters or return values, unlike `fn` functions.
This is because functions are part of an explicit interface 
and their type signatures must be known at the time of declaration, especially when they’re public.
Closures, on the other hand, are often used locally - stored in variables or passed as arguments - 
and aren't typically exposed as part of a public API.
As a result, the compiler can infer their types from context, so explicit annotations are usually unnecessary.

Compare function and closure definitions with the same behavior
```rust
fn add_one_v1(x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // with the type annotation
let add_one_v3 = |x| { x + 1 }; // without the type annotation
let add_one_v4 = |x| x + 1 ; // The closure body has only one expression
```

For closure definitions, the compiler infers 
a single concrete type for each of their parameters and their return value.
```rust
let example_closure = |x| x;
let s = example_closure(String::from("hello")); // `String` inferred for `x`
// compilation error: "error[E0308]: mismatched types"
// let n = example_closure(5);
//         --------------- ^- help: try using a conversion method: `.to_string()`
//         |               |
//         |               expected `String`, found integer
//         arguments to this function are incorrect
```

Closures can capture values from their environment in three ways:
- borrowing immutably
- borrowing mutably
- taking ownership

Put the `move` keyword at the beginning of the closure definition to move ownership of value.