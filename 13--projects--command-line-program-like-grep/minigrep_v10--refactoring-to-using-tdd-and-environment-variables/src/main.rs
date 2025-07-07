/*
```
$ cd 13*
$ cd minigrep_v10*
```

Create a program like grep, (g)lobally search a (r)egular (e)xpression and (p)rint.

Run the program with two arguments:
the first one is a search word and the second one is a file.
Two hyphens (`--`) indicates that the arguments are for the program rather than for Cargo.
```
$ cargo run -- to poem.txt
...
Are you nobody, too?
How dreary to be somebody!
$ IGNORE_CASE=1 cargo run -- to poem.txt
...
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

In PowerShell
```
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
...
PS> Remove-Item Env:IGNORE_CASE
```

Test the library
```
$ cargo test
...
running 2 tests
test tests::case_sensitive ... ok
test tests::case_insensitive ... ok
...
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
