/*
```
$ cd 09*
$ cd 09-5*
$ cargo new project
$ cd project
$ cargo run
```
 */
use std::fs::{self, File};
use std::io::{self, Read};

// The next 4 functions are equivalent.

// Variant 1:
// If this function succeeds without any problems,
// the code that calls this function will receive an `Ok` value
// that holds a `String`.
// If this function encounters any problems,
// the calling code will receive an `Err` value
// that holds an instance of `io::Error`.
fn read_username_from_file_v1() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt"); // Returns `Result<File, io::Error>`
    // Handle `Result` by using `match`
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // Stop the execution by returning `Err(e)`
    };

    let mut username = String::new();

    // The `read_to_string` method returns a `Result<usize, io::Error>`
    // and `Ok(n)` means it successfully read n bytes into the string.
    match file.read_to_string(&mut username) {
        /*
        Ok(n) => Ok(username),  // `n` stores the byte count
         */
        // but
        /*
        Ok(_) => Ok(username),  // byte count is matched but ignored
         */
        // We cannot access `_` later.
        // In Rust, the underscore `_` is a *wildcard pattern*.
        // It means: "I don't care about the value here — I just want to match the shape."
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    // Return either `Ok` or `Err`
}

// Variant 2:
// Use the `?` operator
fn read_username_from_file_v2() -> Result<String, io::Error> {
    // If the value of the `Result` is an `Ok`,
    // the value inside the `Ok` will get returned from this expression,
    // and the program will continue.
    // If the value is an `Err`,
    // the `Err` will be returned from the whole function as
    // if we had used the `return` keyword
    // so the error value gets propagated to the calling code.
    let mut file = File::open("hello.txt")?;
    // Error values that have the `?` operator called on them
    // go through the `from` function,
    // defined in the `From` trait in the standard library.
    // When the `?` operator calls the `from` function,
    // the error type received is converted into the error type
    // defined in the return type of the current function as if
    /*
    Err(err) => return Err(From::from(err)),
     */
    // We need also for converting the error types
    /*
    impl From<io::Error> for SomeOtherError
     */

    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Variant 3:
// Shorten the code by chaining method calls
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Variant 4:
// Make this even shorter using `fs::read_to_string`
fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // All the calls return the same result
    read_username_from_file_v1();
    read_username_from_file_v2();
    read_username_from_file_v3();
    read_username_from_file_v4();
}
