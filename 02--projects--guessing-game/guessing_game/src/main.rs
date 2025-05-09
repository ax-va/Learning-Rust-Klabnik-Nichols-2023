/*
```
$ cd 02*
$ cargo new guessing_game
    Creating binary (application) `guessing_game` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd guessing_game
...
$ cargo run
```
Notice:
The player needs to use binary search to quickly guess a number.
*/

// Import the `io` library from the *standard library*, known as `std`,
// because it is not in the *prelude*, i.e. a set of items
// automatically defined in the scope of every program.
use std::io;
// Import `Ordering` to compare values
use std::cmp::Ordering;
// The `Rng` trait defines methods that random number generators implement.
// The `rand` crate should be added in `Cargo.toml` in the dependencies.
use rand::Rng;

// The `main` function is the entry point into every program
fn main() {
    // `println!` is a macro that prints a string to the screen
    println!("Guess the number!");

    // `rand::thread_rng` returns the particular random number generator
    // that is local to the current thread of execution and
    // is seeded by the operating system.
    let secret_number = rand::thread_rng()
        // `gen_range` takes a range expression as an argument and generates a random number in the range.
        // The kind of range expression we are using here takes the form `<start>..=<end>` and
        // is *inclusive on the lower and upper bounds*.
        // The number is of type `i32`, i.e. it is a 32-bit integer.
        .gen_range(1..=100);

    loop {
        println!("Please input your guess:");

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
        // We could write `std::io::stdin().<function_chain>` without importing `std::io`.
        // `stdin` returns an instance of `std::io::Stdin` to handle the standard input.
        std::io::stdin()
            // The full job of `read_line` is to take whatever the user types into
            // standard input and append that into a string
            // (without overwriting its contents - the string was empty).
            // The `&` indicates that this argument is a *reference*.
            // Like variables, references are immutable by default.
            // Writing `&mut guess` instead of `&guess` makes the reference mutable.
            .read_line(&mut guess)
            // `read_line` returns a `Result` value.
            // `Result` is an *enumeration*, often called an *enum*.
            // The purpose of these `Result` types is to encode error-handling information.
            // `Result`'s variants are `Ok` and `Err`:
            //      1. Inside `Ok` is the successfully generated value;
            //      2. `Err` contains information about how or why the operation failed.
            // If this instance of `Result` is an `Err` value,
            // `expect` will cause the program to crash and display the message.
            // If this instance of `Result` is an `Ok` value,
            // `expect` will take the return value that `Ok` is holding.
            .expect("Failed to read line");
            // Without using `expect`, the program would compile with a warning
            // indicating that the program hasn't handled a possible error.

        // Convert the input of type `String` to `i32` (a signed 32-bit integer)
        // *shadowing* the `guess` variable to reuse it.
        let guess: i32 = match guess
            // Eliminate any whitespace at the beginning and end because
            // if the user types 5 and presses ENTER, `guess` is either "5\n" or "5\r\n" on Windows.
            .trim()
            // `parse` returns an instance of type `Result`
            .parse() {
                // first arm with pattern to match
                Ok(num) => num, // returns unwrapped value
                // second arm with pattern to match: match all `Err` values
                Err(_) => continue, // starts the next iteration
            };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        // The `{}` is used in text as a placeholder.
        // Examples:
        /*
        ```
        let x = 5;
        let y = 10;
        println!("x = {x} and y + 2 = {}", y + 2);
        ```
        */
        println!("You guessed: {guess}");

        // A *match* expression is made up of *arms*.
        // An arm consists of a *pattern* to match against, and
        // the code that should be run if the value given to match fits that arm's pattern.
        // `cmp` returns either `Ordering::Less` or `Ordering::Greater` or `Ordering::Equal`.
        // `match` gets one of these values and starts checking arm's pattern.
        match guess.cmp(&secret_number) {
            // Check if obtained `Ordering::<X>` matches the current arm's patterns `Ordering::<Y>`,
            // execute the associated code in the arm,
            // ends after the first successful match.
            // first arm
            Ordering::Less => println!("Too small!"),
            // second arm
            Ordering::Greater => println!("Too big!"),
            // third arm
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
