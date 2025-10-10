use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| { // closure
        for i in 1..100 {
            println!("Spawned thread: i = {i}");
            // Sleep the current thread for a short time; this gives other threads a chance to run
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Main thread: i = {i}");
        // Sleep the current thread for a short time; this gives other threads a chance to run
        thread::sleep(Duration::from_millis(1));
    }
    // When the main thread of a Rust program completes,
    // all spawned threads are shut down,
    // whether or not they have finished running.
}
// Main thread: i = 1
// Spawned thread: i = 1
// Main thread: i = 2
// Spawned thread: i = 2
// Main thread: i = 3
// Spawned thread: i = 3
// Main thread: i = 4
// Spawned thread: i = 4
// Main thread: i = 5
// Spawned thread: i = 5
// Main thread: i = 6
// Spawned thread: i = 6
// Main thread: i = 7
// Spawned thread: i = 7
// Main thread: i = 8
// Spawned thread: i = 8
// Main thread: i = 9
// Spawned thread: i = 9
// Spawned thread: i = 10
