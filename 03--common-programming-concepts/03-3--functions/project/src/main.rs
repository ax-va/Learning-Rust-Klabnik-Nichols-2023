/*
```
$ cd 03*
$ cd 03-3*
$ cargo new project
$ cd project
$ cargo run
```

Notices:

- Rust code uses *snake case* as the conventional style for function and
variable names.

- Rust doesn't care where functions are defined,
only that they're defined somewhere in a *scope* that can be seen by the caller.

- *Statements* are instructions that perform some action and do not return a value.
Function definitions are also statements.

- *Expressions* evaluate to a resultant value.
Expressions can be part of statements.
Calling a function is an expression.
Calling a macro is an expression.
Expressions do not include ending semicolons.

- You can return early from a function by using the `return` keyword and specifying a value,
but most functions return the last expression *implicitly*.

 */

// In function signatures,
// you *must* declare the type of each parameter
// and also a return type if the function returns a value.

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // This is an expression
    println!("Hello, world!");
    // Hello, world!

    // These are also expressions
    another_function(5);
    // The value of x is: 5
    print_labeled_measurement(5, 'h');
    // The measurement is: 5h

    // This is a statement, but `6` is an expression
    let y = 6;

    // compilation errors: "expected expression, found `let` statement"
    // let x = (let y = 6);
    //          ^^^
    // compilation errors: "expected expression, found `let` statement"
    // let x = let y = 6;
    //         ^^^

    let y = {
        let x = 3;
        // Expressions do not include ending semicolons
        x + 1
    };
    println!("The value of y is: {y}");
    // The value of y is: 4

    let x = five();
    println!("The value of x is: {x}");
    // The value of x is: 5

    let x = plus_one(5);
    println!("The value of x is: {x}");
    // The value of x is: 6
}
