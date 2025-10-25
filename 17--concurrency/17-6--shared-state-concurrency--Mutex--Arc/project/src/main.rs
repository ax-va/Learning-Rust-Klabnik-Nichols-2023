/*
- Mutex = mutual exclusion = only one thread can access the data at a time

Two mutex rules:

1. You must attempt to acquire the lock before using the data.

2. When you're done with the data that the mutex guards,
you must unlock the data so other threads can acquire the lock.

`Mutex<T>` provides interior mutability like `RefCell<T>`.

- `Arc<T>` type = *atomically reference-counted* type

In Rust, atomic types in the `std::sync::atomic` module resemble primitive numeric types
but incorporate atomic operations and ordering semantics.
They implement `Sync`, so they may be safely shared among threads (e.g., via `Arc<AtomicUsize>`).
However, using them correctly still requires reasoning about memory ordering and concurrency;
they do not automatically obviate all concurrency hazards beyond the atomicity guarantees.
 */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    // 1. `Mutex<T>` in a single-threaded context

    let m = Mutex::new(5);

    {
        // The `lock()` method tries to acquire the mutex lock:
        // - if successful, it returns a `Result<MutexGuard<T>, PoisonError>`.
        // - `unwrap()` is called here, which will panic if the mutex is poisoned (not the case here).
        let mut num = m.lock().unwrap(); // The result is a `MutexGuard<i32>`.
        *num = 6;
    } // The guard `num` goes out of scope at the end of the block, automatically releasing the lock

    // `Mutex<T>` implements `Debug` if `T` does.
    // Since `i32` implements `Debug`, this works.
    println!("m = {:?}", m);
    // m = Mutex { data: 6, poisoned: false, .. }

    // 2. `Mutex<T>` between multiple threads

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }); // `num` goes out of scope and releases the lock
        handles.push(handle);
    }

    // Make sure all the threads finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    // Result: 10
}
