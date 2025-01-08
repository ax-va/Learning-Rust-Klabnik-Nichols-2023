/*

Variables of different types are copied in different ways in Rust.

```
$ cd 04*
$ cd 04-1*
$ cargo new project
$ cd project
```
 */

fn main() {

    let x = 5;
    // Types such as integers that have a known size
    // at compile time are stored entirely on the stack.
    let y = x;
    // We now have two variables, `x` and `y`, and both equal 5.
    println!("x = {x}, y = {y}");
    // x = 5, y = 5

    // Notices:
    // - Rust has a special annotation called the `Copy` trait for types
    // that are stored on the stack, as integers are.
    // - If a type implements the `Copy` trait, variables that use it do not move, but rather
    // are trivially copied, making them still valid after assignment to another variable.
    // - Rust won't let us annotate a type with `Copy` if the type, or any of its parts,
    // has implemented the `Drop` trait.

    let s1 = String::from("hello");
    /*

    A `String` is made up of three parts:
    - a pointer to the memory that holds the contents of the string,
    - a length = the memory of the contents in bytes, and
    - a capacity = the total amount of memory in bytes received from the allocator.

    - part of data stored on the stack:

    | name     | value                                |
    |----------|--------------------------------------|
    | ptr      | <pointer to the index 0 on the heap> |
    | len      | 5                                    |
    | capacity | 5                                    |

    - part of data stored on the heap:

    | index | value |
    |-------|-------|
    | 0     | `h`   |
    | 1     | `e`   |
    | 2     | `l`   |
    | 3     | `l`   |
    | 4     | `o`   |

     */

    // *Move* the variable that means,
    // copy the pointer, the length, and the capacity to the memory on the stack, and
    // don't copy the data on the heap that the pointer refers to.
    let s2 = s1;
    // `s1` was invalidated that means, it is no longer valid.
    // We say: `s1` was *moved* into `s2`.

    // compilation error: "error[E0382]: borrow of moved value: `s1`"
    // println!("{s1}, world!");
    //           ^^^^ value borrowed here after move

    // Rust will never automatically create "deep" copies.
    // Use a common method called `clone` to deeply copy the heap data, not just the stack data.

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // The heap data does get copied
    println!("s1 = {s1}, s2 = {s2}");
    // s1 = hello, s2 = hello
}
