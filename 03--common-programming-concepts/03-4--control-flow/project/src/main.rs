/*
```
$ cd 03*
$ cd 03-4*
$ cargo new project
$ cd project
$ cargo run
```

Notices:

- Blocks of code associated with the conditions in the `if` expressions are sometimes called *arms*.

- A condition in control flow code *must* be a `bool`.
Unlike languages such as Python and JavaScript,
Rust *will not* automatically try to convert non-Boolean types to a Boolean.

- Use a powerful Rust branching construct called `match` instead of many `if ... else if`.

- `if ... else` is an expression, hence it can be assigned the outcome to a variable in a `let` statement.
But all arms must return a value of the same type.

- *loops* run through the code inside the loop body to the end and then start immediately back at the beginning.
Rust has three kinds of loops: `loop`, `while`, and `for`.

- In the case of nested loops, `break` and `continue` apply to the *innermost* loop at that point.
Optionally specify a *loop label* on a loop to specify
that the `break` and `continue` keywords are applied to the labeled loop instead of the innermost loop.

 */

fn main() {
    //////////////////////
    // `if` expressions //
    //////////////////////

    let number = 3;

    // The condition in this code *must* be a `bool`
    if number < 5 {
        println!("condition was true");
        // condition was true
    } else {
        println!("condition was false");
    }

    // compilation error: "mismatched types"
    // if number {
    //    ^^^^^^ expected `bool`, found integer
    //    println!("number was three");
    //}

    //////////////////////
    // `if ... else if` //
    //////////////////////

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
        // number is divisible by 3
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    ///////////////////////////////////
    // `if .. else` as an expression //
    ///////////////////////////////////

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    // The value of number is: 5

    // All arms must return a value of the same type

    // compilation error: "`if` and `else` have incompatible types"
    // let number = if condition { 5 } else { "six" };
    //                                        ^^^^^ expected integer, found `&str`

    /////////////////////
    // loops by `loop` //
    /////////////////////

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // This value will be returned out of the loop
        }
    }; // Use a semicolon to end the statement
    println!("The result is {result}");
    // The result is 20

    ///////////////////
    // labeled loops //
    ///////////////////

    let mut count = 0;
    'outer_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer_loop; // Exit the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    // count = 0
    // remaining = 10
    // remaining = 9
    // count = 1
    // remaining = 10
    // remaining = 9
    // count = 2
    // remaining = 10
    println!("End count = {count}");
    // End count = 2

    //////////////////////
    // loops by `while` //
    //////////////////////

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    // 3!
    // 2!
    // 1!
    println!("LIFTOFF!!!");
    // LIFTOFF!!!

    ////////////////////
    // loops by `for` //
    ////////////////////

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    // the value is: 10
    // the value is: 20
    // the value is: 30
    // the value is: 40
    // the value is: 50

    // `rev` is used to reverse the range [1, 4)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    // 3!
    // 2!
    // 1!
    println!("LIFTOFF!!!");
    // LIFTOFF!!!
}
