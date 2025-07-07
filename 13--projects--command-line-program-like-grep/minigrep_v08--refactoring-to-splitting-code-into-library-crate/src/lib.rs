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
    println!("Content:\n{contents}");

    Ok(())
}
