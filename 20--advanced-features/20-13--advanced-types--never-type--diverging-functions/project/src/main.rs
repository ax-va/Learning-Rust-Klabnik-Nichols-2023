/*
The never type `!` represents computations that never return and
is used as the return type of functions that do not complete normally.
 */

// Example 1

// Functions that return never are called *diverging functions*
fn foo() -> ! {
    panic!("This never returns.");
}

// compilation error: "error[E0308]: mismatched types"
/*
fn bar() -> ! {
   // returns the empty tuple `()`
}
*/

// Example 2

fn main() {
    for text in ["10", "abc", "30"] {
        let number: i32 = match text.parse() {
            Ok(n) => n,
            Err(_) => continue,
            // `continue` skips this iteration and has type `!`
            // since it does not produce a value.
        };
        println!("Parsed number: {}", number);
    }
    // Parsed number: 10
    // Parsed number: 30
}