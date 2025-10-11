use std::thread;
use std::time::Duration;

fn with_join_before() {
    // The return type of `thread::spawn` is `JoinHandle<T>`,
    // where `T` is the closure's return type.
    // So the handle type is `JoinHandle<()>`.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread A: i = {i}");
            // Pause the current thread briefly,
            // giving the scheduler a chance to run other threads
            // (no guarantee of strict alternation).
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 1.
    // The main thread will wait for the spawned thread to finish
    // and then run its `for` loop.
    handle.join().unwrap();

    for i in 1..5 {
        println!("Main thread: i = {i}");
        thread::sleep(Duration::from_millis(1));
    }
}

fn with_join_after() {
    let handle = thread::spawn(|| {
        for j in 1..10 {
            println!("Spawned thread B: j = {j}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for j in 1..5 {
        println!("Main thread: j = {j}");
        thread::sleep(Duration::from_millis(1));
    }

    // 2.
    // The two threads continue alternating,
    // but the main thread does not end
    // until the spawned thread is finished.
    handle.join().unwrap();
}

fn main() {
    with_join_before();
    // Spawned thread A: i = 1
    // Spawned thread A: i = 2
    // Spawned thread A: i = 3
    // Spawned thread A: i = 4
    // Spawned thread A: i = 5
    // Spawned thread A: i = 6
    // Spawned thread A: i = 7
    // Spawned thread A: i = 8
    // Spawned thread A: i = 9
    // Main thread: i = 1
    // Main thread: i = 2
    // Main thread: i = 3
    // Main thread: i = 4
    with_join_after();
    // Main thread: j = 1
    // Spawned thread B: j = 1
    // Main thread: j = 2
    // Spawned thread: j = 2
    // Main thread: j = 3
    // Spawned thread B: j = 3
    // Main thread: j = 4
    // Spawned thread B: j = 4
    // Spawned thread B: j = 5
    // Spawned thread B: j = 6
    // Spawned thread B: j = 7
    // Spawned thread B: j = 8
    // Spawned thread B: j = 9
}
