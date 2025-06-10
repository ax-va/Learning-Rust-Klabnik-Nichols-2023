/*
```
$ cd 12*
$ cargo new minigrep
$ cd minigrep
```

Create a program like grep, (g)lobally search a (r)egular (e)xpression and (p)rint.

Run the program with two arguments.
Two hyphens (`--`) indicates that the arguments are for the program rather than for Cargo.
```
$ cargo run -- search_string poem.txt
```
 */
use std::env;
use std::fs;

fn main() {
    // Read any command line arguments passed to it,
    // and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();
    // Notices:
    // 1. `std::env::args` will panic if any argument contains invalid Unicode.
    // 2. We can use the collect function to create many kinds of collections,
    // so we explicitly annotate the type of args to specify that we want a vector of strings.

    // Print the vector using the debug macro
    // dbg!(args);
    /*
    args = [
        "target/debug/minigrep",
        "search_string",
        "poem.txt",
    ]
     */

    // Put a reference to the first argument in the variable `query`
    let query = &args[1];
    // Put a reference to the second argument in the variable `file_path`
    let file_path = &args[2];
    //println!("Searching for '{}' in '{}'...", query, file_path);
    // Searching for 'search_string' in 'poem.txt'...

    // `fs::read_to_string(file_path)` opens the file,
    // and returns an `std::io::Result<String>` of the file's contents.
    let contents = fs::read_to_string(file_path)
        // Set a messages if reading the file leads to panic
        .expect("Should have been able to read the file.");
    println!("Content:\n{contents}");
    /*
    Content:
    I'm nobody! Who are you?
    Are you nobody, too?
    Then there's a pair of us - don't tell!
    They'd banish us, you know.

    How dreary to be somebody!
    How public, like a frog
    To tell your name the livelong day
    To an admiring bog!
     */
}
