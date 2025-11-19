/*
```
$ cd 13*
$ cd minigrep_v06*
```

Create a program like grep, (g)lobally search a (r)egular (e)xpression and (p)rint.

Run the program with two arguments:
the first one is a search word and the second one is a file.
Two hyphens (`--`) indicates that the arguments are for the program rather than for Cargo.
```
$ cargo run -- the poem.txt
```
 */
use std::env;
use std::fs;
use std::process;

fn main() {
    // Read any command line arguments passed to it,
    // and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // This is a closure, i.e. an anonymous function
        println!("Problem with parsing arguments: {err}");
        process::exit(1);
    });

    // `fs::read_to_string(file_path)` opens the `poem.txt` file,
    // and returns an `std::io::Result<String>` of the file's contents.
    let contents = fs::read_to_string(config.file_path)
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

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Prefer `&[String]` over `&Vec<String>`
    // because `&[String]` works with `&Vec<String>`, slices, arrays.
    // Many programmers expect `new` functions to never fail, so we use `build`.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
