/*
```
$ cd 09*
$ cd 09-4*
$ cargo new project
$ cd project
$ cargo run
```
 */
use std::fs::File;
use std::net::IpAddr;

fn main() {
    // If the `Result` value is the `Ok` variant,
    // `unwrap` will return the value inside the `Ok`.
    // If the `Result` is the `Err` variant,
    // `unwrap` will call the `panic!` macro.

    // let greeting_file = File::open("hello.txt").unwrap();
    /*
    thread 'main' panicked at src/main.rs:18:49:
    called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

    // The `expect` method returns the value inside the `Ok` or calls the `panic!` macro
    // but lets us also choose the `panic!` error message.

    // let greeting_file = File::open("hello.txt")
    //     .expect("`hello.txt` should be included in this project.");
    /*
    thread 'main' panicked at src/main.rs:29:10:
    `hello.txt` should be included in this project.: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

    // In production-quality code, most Rustaceans choose `expect` over `unwrap`
    // and give more context about why the operation is expected to always succeed.

    // Having a hardcoded, *valid* value does not change the return `Result` type of methods.
    // It means we should still handle `Ok` and `Err`.
    // In these cases, we can use `unwrap` or even better.
    let home: IpAddr = "127.0.0.1"
        .parse() // returns `Result`
        .expect("Hardcoded IP address should be valid");
}
