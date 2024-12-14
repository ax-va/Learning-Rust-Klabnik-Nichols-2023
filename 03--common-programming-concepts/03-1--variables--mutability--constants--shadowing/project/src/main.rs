/*
We consider variables, mutability, constants, ans shadowing.

```
$ cd 03*
$ cd 03-1*
$ cargo new project
$ cd project
```
*/

fn main() {

    ///////////////
    // variables //
    ///////////////

    // By default, variables are immutable.
    // When a variable is immutable,
    // once a value is bound to a name,
    // the value cannot be changed.
    let x = 5;
    println!("The value of x is: {x}");
    // The value of x is: 5

    // compilation error: "cannot assign twice to immutable variable `x`"
    // x = 6;
    // ^^^^^ cannot assign twice to immutable variable

    println!("The value of x is: {x}");
    // The value of x is: 5

    // mutable variable
    let mut y = 5;
    println!("The value of y is: {y}");
    // The value of y is: 5
    y = 6;
    println!("The value of y is: {y}");
    // The value of y is: 6

    ///////////////
    // constants //
    ///////////////

    // Notices:
    // - Constants uses `const` instead of `let`.
    // - The `mut` keyword is not allowed to use with constants.
    // - The type of the value *must* be annotated.
    // - Constants may be set only to a constant expression,
    // not the result of a value that could only be computed at runtime.
    // But the compiler is able to evaluate a limited set of operations at compile time.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    ///////////////
    // shadowing //
    ///////////////

    // shadowing = effectively creating a new variable when using the `let` keyword again

    // shadowing
    let x = 5;
    // shadowing
    let x = x + 1;

    {
        // other scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // The value of x in the inner scope is: 12
    }

    println!("The value of x is: {x}");
    // The value of x is: 6

    let spaces = "      ";
    // When shadowing, changing types is allowed
    let spaces = spaces.len();

    let mut spaces = "      ";
    // When using mutable variables, changing types results in
    // a compilation error: "mismatched types".
    // spaces = spaces.len();
    //          ^^^^^^^^^^^^ expected `&str`, found `usize`
}
