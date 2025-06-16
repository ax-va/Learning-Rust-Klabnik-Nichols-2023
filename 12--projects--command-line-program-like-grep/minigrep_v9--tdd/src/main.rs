/*
```
$ cd 12*
$ cd minigrep_v9*
```

Create a program like grep, (g)lobally search a (r)egular (e)xpression and (p)rint.

Run the program with two arguments.
Two hyphens (`--`) indicates that the arguments are for the program rather than for Cargo.
```
$ cargo run -- search_string file.txt
$ cargo run -- the poem.txt
```

Test the library
```
$ cargo test
```
 */
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Read any command line arguments passed to it,
    // and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        // This is a body of closure, i.e. a body of an anonymous function
        println!("Problem with parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // This is a body of closure, i.e. a body of an anonymous function
        println!("Application error: {e}");
        process::exit(1);
    }
}
