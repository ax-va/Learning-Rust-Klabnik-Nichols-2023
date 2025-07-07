/*
```
$ cd 12*
$ cd 12-6*
$ cargo new project
$ cd project
$ cargo run
```

*Consuming adapters*
consume the entire iterator to produce a final result,
such as a number, a collection, or a boolean.

Consuming adapters:
- Take ownership of the iterator;
- Consume the iterator entirely by driving it to completion (usually by calling `next` repeatedly);
- After they're called, we cannot use the iterator again;
- Examples: `sum`, `collect`, `for` loops, `fold`, `find`.
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
