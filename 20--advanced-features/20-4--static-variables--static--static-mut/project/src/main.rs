static HELLO_WORLD: &str = "Hello, world!";
// 1. "Hello, world!" is a literal that lives in the program's binary.
// 2. `&str` is an immutable reference (string slice) to UTF-8 text
// stored elsewhere (string literal, String, or other buffer).
// 3. Because it's `static`, that reference is valid for the entire program lifetime.
// 4. `HELLO_WORLD` is a global variable with a fixed memory location.

// Accessing and modifying mutable static variables is *unsafe*.

static mut COUNTER: u32 = 0;
// Any code that reads or writes from `COUNTER` must be within an unsafe block.
// Having multiple threads access `COUNTER` would likely result in data races.

fn add_to_count(inc: u32) {
    unsafe {
        // modifying
        COUNTER += inc;
    }
}

fn main() {
    println!("value is: {HELLO_WORLD}");
    // value is: Hello, world!

    add_to_count(3);
    unsafe {
        // accessing
        println!("COUNTER: {COUNTER}");
        // COUNTER: 3
    }
}
