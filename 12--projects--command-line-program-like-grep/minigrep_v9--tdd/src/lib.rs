use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Prefer `&[String]` over `&Vec<String>`
    // because `&[String]` works with `&Vec<String>`, slices, arrays.
    // Many programmers expect `new` functions to never fail, so we use `build`.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
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

    Ok(())
}

// Indicate that the data returned by the `search` function will live as long as
// the data passed into the search function in the `contents` argument.
pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    vec![]
}

// test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        // This test searches for a line containing "duct"
        let query = "duct";
        // The backslash after the opening double quote tells Rust not to put
        // a newline character at the beginning of the contents of this string literal.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
