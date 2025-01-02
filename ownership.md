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