/*
Mutex = mutual exclusion = only one thread can access the data at a time

Two mutex rules:

1. You must attempt to acquire the lock before using the data.

2. When you're done with the data that the mutex guards,
you must unlock the data so other threads can acquire the lock.
 */
use std::sync::Mutex;

fn main() {
    // `Mutex<T>` in a single-threaded context
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
}
