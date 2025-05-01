/*
```
$ cd 09*
$ cd 09-1*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() {
    // Cause the panic explicitly
    // panic!("crash and burn");
    /*
    thread 'main' panicked at src/main.rs:13:5:
    crash and burn
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
    // The panic has occurred in the 13th line, in the 5th character

    let v = vec![1, 2, 3];
    v[99]; // In C, it is undefined behaviour called *buffer overread* leading to security vulnerabilities
    /*
    thread 'main' panicked at src/main.rs:22:6:
    index out of bounds: the len is 3 but the index is 99
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

     // backtrace
     /*
     $ RUST_BACKTRACE=1 cargo run
     thread 'main' panicked at src/main.rs:22:6:
    index out of bounds: the len is 3 but the index is 99
    stack backtrace:
       0: rust_begin_unwind
                 at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:665:5
       1: core::panicking::panic_fmt
                 at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:74:14
       2: core::panicking::panic_bounds_check
                 at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:276:5
       3: <usize as core::slice::index::SliceIndex<[T]>>::index
                 at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/slice/index.rs:301:10
       4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
                 at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/slice/index.rs:16:9
       5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
                 at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/alloc/src/vec/mod.rs:3323:9
       6: project::main
                 at ./src/main.rs:22:6
       7: core::ops::function::FnOnce::call_once
                 at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/ops/function.rs:250:5
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
      */
}
