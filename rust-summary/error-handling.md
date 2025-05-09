# Error handling

## Unrecoverable and recoverable errors

Instead of exceptions, Rust has 

- the `panic!` macro that stops execution for *unrecoverable errors* 
such as accessing a location beyond the end of an array;

- the type `Result<T, E>` for *recoverable errors* 
such as a file not found error.

When code panics, there is no way to recover.
When you choose to return a `Result` value, you give the calling code options:
either attempt to recover in a way that is appropriate for its situation
or turn your recoverable error into an unrecoverable one.

Functions often have *contracts*: their behavior is only guaranteed 
if the inputs meet particular requirements. 
Panicking when the contract is violated makes sense 
because a contract violation always indicates a caller-side bug, 
and it is not a kind of error you want the calling code to have to explicitly handle.

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

## Propagating Errors

Propagating errors is returning the error to the calling code to handle it later.