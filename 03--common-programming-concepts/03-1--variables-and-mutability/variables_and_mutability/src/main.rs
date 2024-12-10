/*
```
$ cd 03*
$ cd 03-1*
$ cargo new variables_and_mutability
$ cd variables_and_mutability
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

    // Uncomment the next line in to get a compilation error
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

}
