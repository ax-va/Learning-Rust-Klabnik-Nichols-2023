/*
Notices:

- Rust code uses *snake case* as the conventional style for function and
variable names.

- Rust doesn't care where functions are defined, only that
they're defined somewhere in a *scope* that can be seen by the caller.

```
$ cd 03*
$ cd 03-3*
$ cargo new project
$ cd project
```
*/

fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
