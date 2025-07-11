/*
```
$ cd 03*
$ cd 03-2*
$ cargo new project
$ cd project
$ cargo run
```

Rust is a *statically typed* language, which means
that it must know the types of all variables at compile time.

- Types of a known size that can be allocated in the stack:

A *scalar* type represents a single value.
Rust has four primary scalar types:
    - integers,
    - floating-point numbers,
    - Booleans, and
    - characters.

*Compound types* are represented by tuples and arrays.

Use tuples when working with a *small* group of related values with *different* types.
Use arrays when working with a collection of elements of the *same* type.

- Types of an unknown size to allocate in the heap:

The `String` type manages data allocated on the heap and
is able to store an amount of text that is not defined at compile time.

 */

fn main() {

    ///////////////////////////////////////
    // integers and floating-point types //
    ///////////////////////////////////////

    // The compiler can usually infer types of variables.
    // When many types are possible, we must add a type annotation.
    let guess: u32 = "42".parse().expect("Not a number!");

    // compilation error: "type annotations needed"
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

    // Single quotes must be used for the `char` literals,
    // while double quotes must be used for `String` literals.
    let c = 'z';
    // compilation error: "mismatched types"
    // let c: char = "z";
    //               ^^^ expected `char`, found `&str`
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {heart_eyed_cat}");
    // heart_eyed_cat: 😻

    ////////////
    // tuples //
    ////////////

    // The types of the values in the tuple don't have to be the same
    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1); // with optional type annotations

    // Get the individual values out of a tuple.
    // This is called *destructuring*.
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    // The value of x is: 500

    // compilation error: "`(i32, f64, u8)` doesn't implement `std::fmt::Display`"
    // println!("tup: {tup}");
    //                ^^^^^ `(i32, f64, u8)` cannot be formatted with the default formatter

    // Access tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // This indexing requires a *compile-time constant index*
    // because tuples can contain elements of different types,
    // and the compiler needs to know the type of the element at compile time.

    ////////////
    // arrays //
    ////////////

    // An array is a fixed-size collection of elements
    // where all elements must have the same type.
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize an array to contain the same value for each element by specifying the initial value,
    // followed by a semicolon, and then the length of the array in square brackets.
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    // compilation error: "`[{integer}; 5]` doesn't implement `std::fmt::Display`"
    // println!("a: {a}");
    //              ^^^ `[{integer}; 5]` cannot be formatted with the default formatter

    // Access array elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    ////////////
    // String //
    ////////////

    // Calling `String::from` requests the memory on the heap.
    let s = String::from("hello"); // "hello" is an immutable string literal

    // mutable string
    let mut s = String::from("hello");
    // Append a literal to a String
    s.push_str(", world!!!");
    println!("{s}");
    // hello, world!!!
}
