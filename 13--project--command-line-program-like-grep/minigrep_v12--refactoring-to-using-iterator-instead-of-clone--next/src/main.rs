/*
```
$ cd 13*
$ cd minigrep_v12*
```

Create a program like grep, (g)lobally search a (r)egular (e)xpression and (p)rint.

Run the program with two arguments:
the first one is a search word and the second one is a file.
Two hyphens (`--`) indicates that the arguments are for the program rather than for Cargo.

- Redirect the standard output stream to a file
```
$ cargo run -- to poem.txt > output1.txt
...
$ cargo run > output2.txt
```

 */
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Read any command line arguments passed to it,
    // and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();

    // The `env::args` function returns an iterator.
    // We are passing ownership of the iterator
    // returned from `env::args` to `Config::build` directly.
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        // This is a body of closure, i.e. a body of an anonymous function
        eprintln!("Problem with parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // This is a body of closure, i.e. a body of an anonymous function
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
