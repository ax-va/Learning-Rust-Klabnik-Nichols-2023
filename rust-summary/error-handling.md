# Error handling

## Recoverable and unrecoverable errors

Instead of exceptions, Rust has 

- the type `Result<T, E>` for *recoverable errors* 
such as a file not found error;

- the `panic!` macro that stops execution for *unrecoverable errors* 
such as accessing a location beyond the end of an array.

## Unwinding vs. aborting

By default, when a panic occurs the program starts *unwinding*, 
which means Rust walks back up the stack and cleans up the data from each function it encounters.

Alternatively, Rust allows you to set immediately *aborting*, 
which ends the program without cleaning up.
Memory that the program was using should then be cleaned up by the operating system.

```toml
[profile.release]
panic = 'abort'
```

## Backtrace

```unix
$ RUST_BACKTRACE=1 cargo run
```

## The `Result` enum

The `Result` enum is defined as having two variants, `Ok` and `Err`,
with the `T` and `E` generic type parameters, as follows

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` represents the type of the value 
that will be returned in a success case within the `Ok` variant, 
and `E` represents the type of the error 
that will be returned in a failure case within the `Err` variant.
