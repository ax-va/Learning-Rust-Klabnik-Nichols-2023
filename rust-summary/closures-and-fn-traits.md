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
- borrowing immutably,
- borrowing mutably, and
- taking ownership.

Taking ownership:
- Using `move ||` *forces* the closure to capture *all* used variables by value (move), regardless of how they're used;
- Without `move`, Rust automatically infers how a closure needs to capture a variable based on 
how it's used inside the body.

### Fn Traits

A closure's body can do one of the following with captured values:
- Move a captured value out of the closure; 
- Mutate a captured value;
- Neither move nor mutate any captured value;
- Capture nothing from the environment at all.

Notice: Functions can implement all three of the Fn traits too.

Depending on how the closure interacts with captured values, 
it will automatically implement one or more of the three Fn traits 
- `FnOnce`, `FnMut`, and `Fn` - in an additive manner:

#### `FnOnce`

Implemented by all closures, `FnOnce` represents closures that can be called *at least once*.

A closure that moves a captured value out of its body can only be called *once*, 
and therefore implements only `FnOnce`.
When a closure moves a captured variable (i.e., takes ownership of it), 
that value is consumed the first time the closure is called.
Rust's ownership system doesn't allow us to use (or "call") the closure again 
because the moved value no longer exists in the closure - it was taken out. 
We can't move the same value twice.

```rust
fn main() {
    let s = String::from("hello");

    let consume = move || { // `s` is moved *into* the closure
        println!("{}", s); // Use `s`
        s // Return `s`
    }; // `s` is moved out
    
    let s = consume(); // OK: First call works; `s` is moved out and returned
    // hello
    
    // - That first call moves `s` out of the closure's environment.
    // - After that, the closure can no longer be called, because its internal state (the captured `s`) is gone.
    // - The `s` in `let s = consume();` is a new variable, shadowing the earlier one.
    
    // consume(); // Error: Compile-time error: closure was already used (moved)
}
```

#### `FnMut`

Closures that don't move captured values but may mutate them implement `FnMut` (and also `FnOnce`).
These can be called multiple times but may change their internal state.

```rust
let mut counter = 0;

// Closure that mutates `counter` each time it's called
let mut increment = || {
    counter += 1;
    println!("counter = {}", counter);
};

// Call the closure multiple times
increment(); 
// counter = 1
increment(); 
// counter = 2
increment(); 
// counter = 3
```

#### `Fn`

Closures that neither move nor mutate captured values - or capture nothing at all - implement `Fn` 
(and thus also `FnMut` and `FnOnce`).
They can be safely called multiple times, even concurrently, since they don't alter their environment.

```rust
let name = String::from("Sasha");

// Closure that only *reads* the captured value
let greet = || {
    println!("Hello, {}!", name);
};

// Call it multiple times
greet(); 
// Hello, Sasha!
greet(); 
// Hello, Sasha!
greet(); 
// Hello, Sasha!
```

#### Example: Definition of `unwrap_or_else`

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```
where `f` is a `FnOnce` closure.

#### Example: `sort_by_key` requires `FnMut`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
 }
```

The code does not compile because the closure implements only the `FnOnce` trait, 
whereas `sort_by_key` requires a closure that implements `FnMut`. 
The closure captures `value` and then moves `value` out of the closure 
by transferring ownership of `value` to the `sort_operations` vector. 
The closure only implements `FnOnce` because it moves the captured variable `value`, 
which prevents it from being called more than once.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },  
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        sort_operations.push(value); // `value` is moved to `sort_operations`
        r.width
    });
    println!("{:#?}", list);
}
```

Fix it
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },  
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
```
