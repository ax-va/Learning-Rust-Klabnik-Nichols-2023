/*
```
$ cd 13*
$ cd 13-2*
$ cargo new project
$ cd project
$ cargo run
```

In Rust, closures usually don't require you to annotate
the types of their parameters or return values, unlike `fn` functions.
This is because functions are part of an explicit interface
and their type signatures must be known at the time of declaration, especially when they’re public.
Closures, on the other hand, are often used locally - stored in variables or passed as arguments -
and aren't typically exposed as part of a public API.
As a result, the compiler can infer their types from context, so explicit annotations are usually unnecessary.

Compare function and closure definitions with the same behavior
```
fn add_one_v1(x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // with the type annotation
let add_one_v3 = |x| { x + 1 }; // without the type annotation
let add_one_v4 = |x| x + 1 ; // The closure body has only one expression
```
 */

fn main() {
    // For closure definitions, the compiler infers
    // a single concrete type for each of their parameters and their return value.
    let example_closure = |x| x; // `String` inferred for `x`
    let s = example_closure(String::from("hello"));
    // compilation error: "error[E0308]: mismatched types"
    // let n = example_closure(5);
    //         --------------- ^- help: try using a conversion method: `.to_string()`
    //         |               |
    //         |               expected `String`, found integer
    //         arguments to this function are incorrect

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        num
    };
    expensive_closure(10);
    // calculating slowly...
}
