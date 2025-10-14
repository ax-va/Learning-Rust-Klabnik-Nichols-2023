use std::thread;

fn main() {
    // To use data from the main thread in the spawned thread,
    // the spawned thread's closure must capture the values it needs.
    let v = vec![1, 2, 3];

    // compilation error:
    // "error[E0373]: closure may outlive the current function,
    // but it borrows `v`, which is owned by the current function"

    // let handle = thread::spawn(|| {
    // //                         ^^ may outlive borrowed value `v
    //     println!("Here's a vector: {:?}", v);
    // });
    // handle.join().unwrap();

    // Explanation:
    // The closure tries to borrow `v`.
    // But Rust can't know how long the spawned thread will run,
    // so it doesn't know whether the reference to `v` will always be valid.

    // Force the closure to take ownership of the values it's using.
    // The main thread can't use `v` anymore.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // compilation error: "error[E0382]: use of moved value: `v`"
    // drop(v); // oh no!
    // //   ^ value used here after move

    handle.join().unwrap();


}
// Here's a vector: [1, 2, 3]
