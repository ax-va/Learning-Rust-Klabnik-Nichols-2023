/*
The `fn` type is a type whose values are pointers to functions,
called a *function pointer* type.

It allows passing behavior as data.

Function pointer types:
- are sized;
- implement `Copy`;
- implement `Clone`;
- implement all three of the closure traits (`Fn`, `FnMut`, and `FnOnce`)
 -> A function pointer can be passed as an argument for a function
 that expects a closure.
 */

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // `fn(i32) -> i32` is the type of `f`
    f(arg) + f(arg)
}

fn main() {
    // `add_one` is a function item,
    // but it automatically converts to
    // the function pointer type `fn(i32) -> i32`.
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");
    // The answer is: 12
}
