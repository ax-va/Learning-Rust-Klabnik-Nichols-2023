/*
```
$ cd 12*
$ cargo new minigrep_v2
$ cd minigrep_v2*
```

Create a program like grep, (g)lobally search a (r)egular (e)xpression and (p)rint.

Run the program with two arguments.
Two hyphens (`--`) indicates that the arguments are for the program rather than for Cargo.
```
$ cargo run -- search_string file.txt
$ cargo run -- the poem.txt
```
 */
use std::env;
use std::fs;

fn main() {
    // Read any command line arguments passed to it,
    // and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    // `fs::read_to_string(file_path)` opens the `poem.txt` file,
    // and returns an `std::io::Result<String>` of the file's contents.
    let contents = fs::read_to_string(file_path)
        // Set a message if reading the file leads to panic
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

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
