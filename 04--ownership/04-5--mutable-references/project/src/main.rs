/*

If you have a mutable reference to a value,
using other (mutable or immutable) references to that value is *restricted*.

The benefit of having this restriction is that
Rust can prevent *data races* at compile time that
happen when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There is no mechanism being used to synchronize access to the data.

```
$ cd 04*
$ cd 04-5*
$ cargo new project
$ cd project
```
 */

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
    // hello, world

    // Two mutable references can be created in the same scope,
    // but they cannot be used in that scope *simultaneously* or *in a mixed manner*.

    let r1 = &mut s;
    // compilation error: "error[E0499]: cannot borrow `s` as mutable more than once at a time"
    // let r2 = &mut s;
    //          ^^^^^^ second mutable borrow occurs here
    // println!("{r1}, {r2}");

    // This is a working example
    // because the two mutable references are *only used one after the other*.
    // Thus the scopes of use of these references do not overlap.
    let r1 = &mut s;
    change(r1);
    println!("{r1}"); // This is the last use of `r1`.
    // hello, world, world
    let r2 = &mut s;
    change(r2); // This is the first use of `r2`.
    println!("{r2}");
    // hello, world, world, world

    // Using two mutable references in a mixed manner leads to the compilation error,
    // for example, by adding:
    // println!("{r1}");

    // Cannot *use* a mutable reference with immutable references *simultaneously* or *in a mixed manner* either
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1}, {r2}");
    // hello, hello

    // compilation error: "error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable"
    // let r3 = &mut s; // PROBLEM
    //          ^^^^^^ mutable borrow occurs here
    // println!("{r1}, {r2}, {r3}");

    // The compilation error occurs also here:
    // let r3 = &mut s; // PROBLEM
    // println!("{r1}, {r2}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}