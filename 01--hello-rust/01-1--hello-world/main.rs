// The `main` function is always an entry point to every executable Rust program
fn main() {
    // 1. Rust style is to indent with four spaces, not a tab.
    // 2. `println!` calls a Rust macro: "!" is for a macro.
    // Notice: macros do not always follow the same rules as functions.
    println!("Hello, world!");
}

// Compile the file and run the program on Linux and macOS
/*
$ cd 01--hello-rust/01-1--hello-world
$ rustc main.rs
$ ls
main  main.rs
$ ./main
Hello, world!
*/

// on Windows
/*
> rustc main.rs
> .\main.exe
*/
