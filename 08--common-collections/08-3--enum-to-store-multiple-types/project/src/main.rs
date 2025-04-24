/*
```
$ cd 08*
$ cd 08-3*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() {

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }


    let row = vec![
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("done")),
    ];

    // Using an enum with a `match` expression allows Rust to ensure at compile time
    // that every possible case is handled.
}
