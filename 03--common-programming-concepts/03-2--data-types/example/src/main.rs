/*
Rust is a *statically typed* language, which means that
it must know the types of all variables at compile time.

A *scalar* type represents a single value.
Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters.

*Compound types* are represented by tuples and arrays.

```
$ cd 03*
$ cd 03-2*
$ cargo new example
$ cd example
```
*/

fn main() {

    ///////////////////////////////////////
    // integers and floating-point types //
    ///////////////////////////////////////

    // The compiler can usually infer types of variables.
    // When many types are possible, we must add a type annotation.
    let guess: u32 = "42".parse().expect("Not a number!");

    // compilation error
    // let guess = "42".parse().expect("Not a number!");
    //     ^^^^^        ----- type must be known at this point

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    ////////////////////////
    // numeric operations //
    ////////////////////////

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;
    println!("difference: {difference}");
    // difference: 91.2

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    // quotient: 1.7608695652173911

    let truncated = 5 / 3; // results in 1
    println!("truncated: {truncated}");
    // truncated: 1

    let truncated = -5 / 3; // results in -1
    println!("truncated: {truncated}");
    // truncated: -1

    let remainder = 43 % 5;
    println!("remainder: {remainder}");
    // remainder: 3

    //////////////
    // Booleans //
    //////////////

    let t = true;
    let f: bool = false; // with explicit type annotation

    ////////////////
    // characters //
    ////////////////

    // Single quotes must be used for the `char` literals, while
    // double quotes must be used for `String` literals.
    let c = 'z';
    // compilation error
    // let c: char = "z";
    //               ^^^ expected `char`, found `&str`
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {heart_eyed_cat}");
    // heart_eyed_cat: 😻

    ////////////
    // tuples //
    ////////////

    // The types of the different values in the tuple don't have to be the same
    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1); // with optional type annotations

    // Get the individual values out of a tuple.
    // This is called destructuring.
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    // The value of x is: 500

    // compilation error
    // println!("tup: {tup}");
    //                ^^^^^ `(i32, f64, u8)` cannot be formatted with the default formatter

    // Access a tuple element directly by using a period followed by an index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    ////////////
    // arrays //
    ////////////

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize an array to contain the same value for each element by specifying the initial value,
    // followed by a semicolon, and then the length of the array in square brackets.
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    // compilation error
    // println!("a: {a}");
    //              ^^^ `[{integer}; 5]` cannot be formatted with the default formatter
}
