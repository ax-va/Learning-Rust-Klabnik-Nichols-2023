# Error handling

## Recoverable and unrecoverable errors

Instead of exceptions, Rust has the type `Result<T, E>` for *recoverable errors*
such as a file not found error and the `panic!` macro that stops execution for *unrecoverable errors* 
such as accessing a location beyond the end of an array.

## Unrecoverable errors

### Unwinding vs. aborting

By default, when a panic occurs the program starts *unwinding*, 
which means Rust walks back up the stack and cleans up the data from each function it encounters.

Alternatively, Rust allows you to set immediately *aborting*, 
which ends the program without cleaning up.
Memory that the program was using should then be cleaned up by the operating system.

```toml
[profile.release]
panic = 'abort'
```

### Backtrace

```unix
$ RUST_BACKTRACE=1 cargo run
```
