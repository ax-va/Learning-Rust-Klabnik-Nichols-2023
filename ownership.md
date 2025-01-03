# Ownership 

*Ownership* is a set of rules that govern how a Rust program manages memory in the *heap*.
If any of the rules are violated, the program won't compile.

## Ownership rules

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable scope

This is an example of variable scope:
```rust
{
    // `s` is not valid here, since it's not yet declared

    let s = "hello"; // `s` is valid from this point forward

    // Do stuff with `s`
    
} // This scope is now over, and `s` is no longer valid
```

- When `s` comes *into* scope, it is valid.
- It remains valid until it goes *out of* scope.
- The allocated memory is automatically returned once the variable that owns it goes out of scope.
- Rust calls the `drop` function automatically at the closing curly bracket `}` to return the memory to the allocator.

## Ownership and Functions

Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello"); // `s` comes into scope.
    takes_ownership(s); // The value of `s` moves into the function.
    // `s` is no longer valid.
    
    let x = 5; // `x` comes into scope.
    makes_copy(x); // The copy of `x` goes into the function.
    // `x` is valid because but `i32` implements `Copy`.
} // Here, `x` goes out of scope, then `s`. 
// However, because the value of `s` was moved, nothing special happens.

fn takes_ownership(some_string: String) { // `some_string` comes into scope.
    println!("{some_string}");
} // Here, `some_string` goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // `some_integer` comes into scope.
    println!("{some_integer}");
} // Here, `some_integer` goes out of scope. Nothing special happens.
```

## Return Values and Scope

```rust
fn main() {
    let s1 = gives_ownership(); // `gives_ownership` moves its return value into `s1`.
    let s2 = String::from("hello"); // `s2` comes into scope.
    let s3 = takes_and_gives_back(s2); // `s2` is moved into `takes_and_gives_back`,
    // which also moves its return value into `s3`.
} // Here, `s3` goes out of scope and is dropped.
// `s2` was moved, so nothing happens. `s1` goes out of scope and is dropped.

// This function will move its return value into the function that calls it.
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // `some_string` comes into scope.
    some_string
    // `some_string` is returned and moves out to the calling function.
}

// This function takes a `String` and returns a `String`.
fn takes_and_gives_back(a_string: String) -> String { // `a_string` comes into scope.
    a_string
    // `a_string` is returned and moves out to the calling function
}
```