/*
The idea of *message passing* in a slogan at https://golang.org/doc/effective_go.html#concurrency
"Do not communicate by sharing memory; instead, share memory by communicating."

To enable message-passing concurrency, Rust's standard library provides *channels*:
- A channel is a common concurrency abstraction, in which one thread sends data and another thread receives it.
- A channel has two halves: a transmitter and a receiver.
- A channel is closed if the receiver is dropped, or if all of its transmitters are dropped.
 */

use std::sync::mpsc; // mpsc = *multiple producer, single consumer*
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // (tx, rx) = (transmitter, receiver)

    // Create a new thread and then
    // move the transmitter `tx` into the closure
    // so that the spawned thread owns `tx`.
    // The spawned thread needs to own the transmitter
    // to be able to send messages through the channel.
    thread::spawn(move || {
        let v1 = String::from("hi");
        let v2 = String::from("again");
        // The `send` method returns a `Result<T, E>` type.
        // Use `unwrap` to panic in case of an error.
        tx.send(v1).unwrap();
        tx.send(v2).unwrap();
    });

    // Get the value from the receiver in the main thread:
    // - The main thread will pause at `recv()` and wait.
    // - The `recv` method returns a `Result<T, E>` type.
    // - If a value arrives, `unwrap()` extracts it.
    // - If the channel is closed before any value is received, `unwrap()` will panic.
    // - See also `try_recv` that immediately returns `Result`, without blocking the current thread.
    let received = rx.recv().unwrap(); // recv = receive
    println!("Got: {received}");
    // Got: hi
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // Got: again
}
