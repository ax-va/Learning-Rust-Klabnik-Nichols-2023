/*
```
$ cd 13*
$ cd 13-6*
$ cargo new project
$ cd project
$ cargo run
```

Methods that call `next` are called *consuming adapters* because they use up the iterator.
(`next` does not consume the iterator itself - it only mutably borrows it to advance its internal state.)
One example is the `sum` method, which takes ownership of the iterator
and consumes it by repeatedly calling next to process each item."

 */

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("Total: {total}");
    // Total: 6

    // compilation: error: "error[E0382]: use of moved value: `v1_iter`"
    // let total: i32 = v1_iter.sum();
    //                  ^^^^^^^ value used here after move
}
