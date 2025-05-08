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
    let file_result = File::open("hello.txt"); // returns a `Result<File, io::Error>`
    // Handle `Result` by using `match`
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // early return of a value out of the function
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
    // We need also this for converting the error types
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

// We can only use the ? operator in a function
// that returns `Result`, `Option`, or another type that implements the `FromResidual` trait.

// The `?` operator called on an `Option<T>`:
// - if the value is `None`, the `None` will be returned early from the function at that point;
// - if the value is `Some`, the value inside the `Some` is the resultant value of the expression,
// and the function continues.

// Example: find the last character of the first line in the given text
fn last_char_of_first_line(text: &str) -> Option<char> {
    // This is an `Option` because it is possible
    // that the first line is the empty string as in "\nHi!".

    // The `lines` method returns an iterator over the lines in the string.
    // - If `text` is the empty string, `next` will return `None`,
    // in which case `?` stops the chain and return `None`.
    // - If `text` is not the empty string, `next` will return a `Some` value
    // containing a string slice of the first line in text.
    // In this case, `chars` on that string slice gets an iterator of its characters.
    // Then, `last` returns the last item in the iterator.
    text.lines().next()?.chars().last()
}

/*
fn main() {
    // All the calls return the same result
    read_username_from_file_v1();
    read_username_from_file_v2();
    read_username_from_file_v3();
    read_username_from_file_v4();

    // misusing the ? operator

    // compilation error:
    // "error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)"
    // let greeting_file = File::open("hello.txt")?;
    //                                            ^ cannot use the `?` operator in a function that returns `()`

    // Explanation:
    // The `?` operator follows the `Result` value returned by `File::open`,
    // but this main function returns the `()` type (the *unit type*), not `Result´.
    // We can only use the `?` operator in a function
    // that returns `Result`, `Option`, or another type
    // that implements the `FromResidual` trait.
}
*/

// Fix that changing the `main` function.

// This trait represents any error type in Rust.
// It allows you to work with different kinds of errors in a generic way.
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // `()` is the *unit type*, representing "no meaningful value" like `void` in other languages.
    // `Box<dyn Error>` means "any kind of error".

    let greeting_file = File::open("hello.txt")?; // returns a `Result<File, std::io::Error>`
    // - If the file opens successfully, the `File` object is stored in `greeting_file`.
    // - If there is an error (e.g., file not found),
    // the error is automatically returned from `main` wrapped in the `Box<dyn Error>`.

    // `Ok(())` is needed at the end of this main function
    // because the function is declared to return a `Result<(), Box<dyn Error>>`.
    // So `Ok(())` means "everything went fine, and there's nothing to return".
    Ok(())
    // The main function may return any types
    // that implement the `std::process::Termination` trait,
    // which contains a function `report` that returns an `ExitCode`.
}
