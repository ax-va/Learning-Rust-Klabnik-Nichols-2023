"""
# Interior mutability

*Interior mutability* is a design pattern in Rust
where a type allows mutation of its internal state
even when accessed through an immutable reference.
This is achieved by enforcing borrowing rules at runtime
rather than compile time (e.g., via `RefCell`, `Cell`, or `Mutex`).

## `unsafe`
`unsafe` in Rust marks code where the compiler
can no longer guarantee memory safety by itself,
and it becomes the programmer's job to uphold certain rules.
We can use interior mutability types when we can ensure
that Rust's borrowing rules are upheld at runtime,
even though the compiler cannot verify them at compile time.

## Borrowing rules
- At any given time, you can have either one mutable reference or any number of immutable references (but not both).
- References must always be valid.

## `Box<T>` vs. `RefCell<T>`
- `Box<T>` (and most normal references):
The compiler enforces the borrowing rules at compile time.
If you try to create an illegal combination of borrows, the code won't compile.
- `RefCell<T>` (and other interior mutability types):
The compiler can't enforce the rules at compile time, so `RefCell enforces them at runtime.
If you violate the borrowing rules, the program will panic immediately and exit.

## Enforcing at runtime
The advantage of checking borrowing rules at runtime is that some memory-safe scenarios become possible
that would otherwise be rejected by Rust’s conservative compile-time analysis.
Static analysis, like the Rust compiler, must be conservative:
some properties of code are impossible to prove purely at compile time.
"""

fn main() {
    // ...
}
