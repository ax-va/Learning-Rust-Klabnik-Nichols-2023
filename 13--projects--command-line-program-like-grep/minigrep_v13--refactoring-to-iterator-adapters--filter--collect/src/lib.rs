use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        // `args` can be any type
        // that implements the `Iterator` trait
        // and returns `String` items.
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // The first item in the return iterator of `env::args` is the name of the program
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path"),
        };
        // The `env::var` function returns a `Result`.
        // If we get `Ok`, `is_ok()` returns `true`.
        // If we get `Err`, `is_ok()` returns `false`.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(
            Config {
                query,
                file_path,
                ignore_case,
            }
        )
    }
}

// Use the *trait object* `Box<dyn Error>` for the error type.
// It is used to return any error type in a uniform way.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `fs::read_to_string(file_path)` opens the `poem.txt` file,
    // and returns an `std::io::Result<String>` of the file's contents.
    // The `?` operator will return the error value
    // from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// Indicate that the data returned by the `search` function will live as long as
// the data passed into the search function in the `contents` argument.
pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // Filter and then collect the matching lines into another vector
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase(); // returned a `String` rather than a string slice
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        // This test searches for a line containing "duct"
        let query = "duct";
        // The backslash after the opening double quote tells Rust not to put
        // a newline character at the beginning of the contents of this string literal.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
