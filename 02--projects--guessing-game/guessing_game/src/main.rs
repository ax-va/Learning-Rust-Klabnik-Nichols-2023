/*
$ cd 02*
$ cargo new guessing_game
    Creating binary (application) `guessing_game` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd guessing_game
...
$ cargo run
*/

// Import the `io` library from the standard library, known as `std`, because it is not in the *prelude*
// (the set of items automatically defined in the scope of every program).
use std::io;

// The `main` function is the entry point into every program
fn main() {
    // `println!` is a macro that prints a string to the screen
    println!("Guess the number!");
    println!("Please input your guess: ");
    // In Rust, variables are immutable by default.
    // To make a variable mutable, we add `mut` before the variable name.
    // Examples:
    /*
    ```
    let apples = 5; // immutable
    let mut bananas = 5; // mutable
    ```
    */
    // `new` is an *associated function* to type `String`
    // that returns a new instance of `String` (encoded as UTF-8)
    // that is an empty string.
    // In full, create a mutable variable
    // that is currently bound to a new, empty instance of a `String`.
    let mut guess = String::new();
    // We can use `std::io::stdin` without importing `std::io`.
    // `stdin` returns an instance of `std::io::Stdin` to handle the standard input.
    io::stdin()
        // The full job of `read_line` is to take whatever the user types into
        // standard input and append that into a string (without overwriting its contents).
        // The `&` indicates that this argument is a *reference*.
        // Like variables, references are immutable by default.
        // Writing `&mut guess` instead of `&guess` makes the reference mutable.
        .read_line(&mut guess)
        // `read_line` returns a `Result` value.
        // `Result` is an *enumeration*, often called an *enum*.
        // The purpose of these `Result` types is to encode error-handling information.
        // `Result`'s variants are `Ok` and `Err`:
        // 1. Inside `Ok` is the successfully generated value;
        // 2. `Err` contains information about how or why the operation failed.
        // If this instance of `Result` is an `Err` value,
        // expect will cause the program to crash and display the message.
        // If this instance of `Result` is an `Ok` value,
        // `expect` will take the return value that `Ok` is holding.
        .expect("Failed to read line");
        // Without using `expect, the program would compile with a warning
        // indicating that the program hasn't handled a possible error.
    // The `{}` set of curly brackets is a placeholder.
    // Example:
    /*
    ```
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
    ```
    */
    println!("You guessed: {guess}");
}
