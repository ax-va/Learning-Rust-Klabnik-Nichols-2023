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
    // Examples: `let apples = 5; // immutable` and `let mut bananas = 5; // mutable`.
    // `new` is an *associated function* to type `String`
    // that returns a new instance of `String` (encoded as UTF-8)
    // that is an empty string.
    // In full, create a mutable variable
    // that is currently bound to a new, empty instance of a `String`.
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
