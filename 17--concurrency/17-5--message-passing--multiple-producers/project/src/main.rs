use std::sync::mpsc; // mpsc = *multiple producer, single consumer*
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Thread 1: hi"),
            String::from("Thread 1: from"),
            String::from("Thread 1: the"),
            String::from("Thread 1: thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Thread 2: more"),
            String::from("Thread 2: messages"),
            String::from("Thread 2: for"),
            String::from("Thread 2: you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    // Got: Thread 1: hi
    // Got: Thread 2: more
    // Got: Thread 1: from
    // Got: Thread 2: messages
    // Got: Thread 1: the
    // Got: Thread 2: for
    // Got: Thread 2: you
    // Got: Thread 1: thread
}
