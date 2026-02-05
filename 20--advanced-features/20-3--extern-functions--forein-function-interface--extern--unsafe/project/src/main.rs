/*
Rust provides the *extern* keyword to support *Foreign Function Interfaces* (FFI),
allowing Rust code to call functions written in other languages and
to expose Rust functions so they can be called from those languages.

It does two things:
- Declare foreign functions/types so Rust can call them (- commonly, Rust calls C libraries);
- Expose Rust functions so other languages can call Rust (- C or C++ can call Rust functions).

So FFI is bidirectional.

Rust's extern is designed primarily around C *Application Binary Interface* (ABI),
because C is the "universal bridge" most languages can talk to.

Python doesn't natively use Rust's ABI, so interaction usually goes through C.
In practice, most Rust developers use PyO3 instead of writing raw extern bindings.
 */

// The ABI defines how to call the function at the assembly level.
// The `extern` block is only a declaration.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        // Absolute value of -3 according to C: 3
    }
}
// On most platforms, `abs` is provided by the C runtime / C standard library.
// When we build a normal Rust program that uses `std`,
// the final link step already links against the platform's C runtime / libc (directly or indirectly).
// So when the linker sees an unresolved symbol named `abs`,
// it searches the linked libraries and finds a matching exported symbol named `abs`.
