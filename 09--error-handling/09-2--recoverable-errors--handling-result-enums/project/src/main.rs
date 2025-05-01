/*
```
$ cd 09*
$ cd 09-2*
$ cargo new project
$ cd project
$ cargo run
```
 */

use std::fs::File;

fn main() {
    // The return type of `File::open` is `Result<T, E>`.
    // The generic parameter `T` has been filled in
    // with the type of the success value, `std::fs::File`.
    // The type of `E` used in the error value is `std::io::Error`.
    let greeting_file_result = File::open("hello.txt");
    // In the case where `File::open` succeeds,
    // the return value is an instance of `Ok`.
    // In the case where it fails,
    // the return value is an instance of `Err`.

    // Handle the `Result` using the `match` expression.
    // `Result`'s `Ok` and `Err` are automatically in the prelude.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
    /*
    thread 'main' panicked at src/main.rs:28:13:
    Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
}
