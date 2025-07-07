/*
```
$ cd 12*
$ cd 12-3*
$ cargo new project
$ cd project
$ cargo run
```

Closures can capture values from their environment in three ways:
- borrowing immutably,
- borrowing mutably, and
- taking ownership.

Taking ownership:
- Using `move ||` *forces* the closure to capture *all* used variables by value (move), regardless of how they're used;
- Without `move`, Rust automatically infers how a closure needs to capture a variable based on
how it's used inside the body.

 */
use std::thread;

fn main() {
    // - A closure captures an immutable reference
    let list1 = vec![1, 2, 3];
    println!("`list1` before defining closure: {:?}", list1);
    // `list1` before defining closure: [1, 2, 3]
    let only_borrows = || println!("`list1` from closure: {:?}", list1);
    println!("`list1` before calling closure: {:?}", list1);
    // `list1` before calling closure: [1, 2, 3]
    only_borrows();
    // `list1` from closure: [1, 2, 3]
    println!("`list1` after calling closure: {:?}", list1);
    // `list1` after calling closure: [1, 2, 3]

    // - A closure captures a mutable reference
    let mut list2 = vec![1, 2, 3];
    println!("`list2` before defining closure: {:?}", list2);
    // `list2` before defining closure: [1, 2, 3]
    let mut borrows_mutably = || list2.push(7);

    // compilation error: "error[E0502]: cannot borrow `list2` as immutable because it is also borrowed as mutable"
    // println!("`list2` before calling closure: {:?}", list2);
    //                                                  ^^^^^ immutable borrow occurs here
    // Explanation: when `borrows_mutably` is defined, it captures a mutable reference to `list2`.

    borrows_mutably();
    println!("`list2` after calling closure: {:?}", list2);
    // `list2` after calling closure: [1, 2, 3, 7]

    // - A closure takes ownership of the values it uses in the environment
    let list3 = vec![1, 2, 3];
    println!("`list3` before defining closure: {:?}", list3);
    // `list3` before defining closure: [1, 2, 3]
    // Spawn a new thread, giving the thread a closure to run as an argument.
    // Specify that `list3` should be moved into the closure by putting the `move` keyword
    // at the beginning of the closure definition.
    thread::spawn(move || {
        println!("`list3` from thread: {:?}", list3)
    }).join().unwrap();
    // `list3` from thread: [1, 2, 3]

    // Notice:
    // If the main thread maintains ownership of `list3`
    // but ends before the new thread and drops `list3`,
    // the immutable reference in the thread would be invalid.

    let list4 = vec![1, 2, 3];
    println!("`list4` before defining closure: {:?}", list4);
    // compilation error: "error[E0373]: closure may outlive the current function, but it borrows `list4`, which is owned by the current function"
    // thread::spawn(|| {
    //               ^^ may outlive borrowed value `list4`
    //     println!("`list4` from thread: {:?}", list4)
    // }).join().unwrap();
}
