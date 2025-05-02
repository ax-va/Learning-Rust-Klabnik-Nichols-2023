/*
```
$ cd 09*
$ cd 09-3*
$ cargo new project
$ cd project
$ cargo run
```
 */
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Handle errors using the `match` expression
    let greeting_file_result = File::open("hello1.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // The `ErrorKind` is enum
            ErrorKind::NotFound => {
                // Try to create the file
                match File::create("hello1.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Alternatively, handle errors using the `unwrap_or_else` method and closures.
    // Try to open the file "hello2.txt".
    // If it succeeds, the file handle is stored in `greeting_file`.
    // If it fails, it enters the `unwrap_or_else` closure, where the `error` is handled.
    // *Closure* (also called a lambda or anonymous or arrow function) syntax in Rust:
    // `|parameters| expression_or_block`, for example, `let add = |a, b| a + b;`.
    // In Rust, closures are anonymous functions
    // that can capture variables from their surrounding environment.
    // But not all closures are necessarily anonymous in other languages.
    // For example, in Python or JavaScript, closures are about capturing state,
    // whether the function is anonymous or not.
    /*
    # in Python
    def outer(x):
        def inner(y):
            return x + y  # `x` is captured
        return inner
     */
    /*
    // in JavaScript
    function outer(x) {
        return function inner(y) {
            return x + y; // `x` is captured
        };
    }
     */
    let greeting_file = File::open("hello2.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello2.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        }
    );
}
