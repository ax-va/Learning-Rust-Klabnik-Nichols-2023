/*
```
$ cd 13*
$ cd minigrep_v07*
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
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // Read any command line arguments passed to it,
    // and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        // This is a body of closure, i.e. a body of an anonymous function
        println!("Problem with parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        // This is a body of closure, i.e. a body of an anonymous function
        println!("Application error: {e}");
        process::exit(1);
    }
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

// Use the *trait object* `Box<dyn Error>` for the error type.
// It is used to return any error type in a uniform way.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `fs::read_to_string(file_path)` opens the `poem.txt` file,
    // and returns an `std::io::Result<String>` of the file's contents.
    // The `?` operator will return the error value
    // from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;
    println!("Content:\n{contents}");

    Ok(())
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
