/*
Rust is a *statically typed* language, which means that
it must know the types of all variables at compile time.

A *scalar* type represents a single value.
Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters.

```
$ cd 03*
$ cd 03-2*
$ cargo new example
$ cd example
```
*/

fn main() {

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

    /////////////
    // Boolean //
    /////////////

    let t = true;
    let f: bool = false; // with explicit type annotation

    ////////////////////////
    // The character type //
    ////////////////////////

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {heart_eyed_cat}");
    // heart_eyed_cat: 😻
}
