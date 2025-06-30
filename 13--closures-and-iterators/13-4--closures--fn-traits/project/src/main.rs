/*
```
$ cd 13*
$ cd 13-4*
$ cargo new project
$ cd project
$ cargo run
```

Fn Traits

A closure's body can do one of the following with captured values:
- Move a captured value out of the closure;
- Mutate a captured value;
- Neither move nor mutate any captured value;
- Capture nothing from the environment at all.

Notice: Functions can implement all three of the Fn traits too.

Depending on how the closure interacts with captured values,
it will automatically implement one or more of the three Fn traits
- `FnOnce`, `FnMut`, and `Fn` - in an additive manner:

- `FnOnce`

Implemented by all closures, `FnOnce` represents closures that can be called *at least once*.

A closure that moves a captured value out of its body can only be called *once*,
and therefore implements only `FnOnce`.
When a closure moves a captured variable (i.e., takes ownership of it),
that value is consumed the first time the closure is called.
Rust's ownership system doesn't allow us to use (or "call") the closure again
because the moved value no longer exists in the closure - it was taken out.
We can't move the same value twice.

- `FnMut`

Closures that don't move captured values but may mutate them implement `FnMut` (and also `FnOnce`).
These can be called multiple times but may change their internal state.

- `Fn`

Closures that neither move nor mutate captured values - or capture nothing at all - implement `Fn`
(and thus also `FnMut` and `FnOnce`).
They can be safely called multiple times, even concurrently, since they don't alter their environment.

 */

fn main() {
    // `FnOnce` //

    let s = String::from("hello");

    let consume = move || { // `s` is moved *into* the closure
        println!("{}", s); // Use `s`
        s // Return `s`
    }; // `s` is moved out

    let s = consume(); // OK: First call works; `s` is moved out and returned
    // hello

    // - That first call moves `s` out of the closure's environment.
    // - After that, the closure can no longer be called, because its internal state (the captured `s`) is gone.
    // - The `s` in `let s = consume();` is a new variable, shadowing the earlier one.

    // compilation error: "error[E0382]: use of moved value: `consume`"
    // consume(); // Error: Compile-time error: closure was already used (moved)
    // ^^^^^^^ value used here after move

    // `FnMut` //

    let mut counter = 0;

    // Closure that mutates `counter` each time it's called
    let mut increment = || {
        counter += 1;
        println!("counter = {}", counter);
    };

    // Call the closure multiple times
    increment();
    // counter = 1
    increment();
    // counter = 2
    increment();
    // counter = 3

    // `Fn` //

    let name = String::from("Sasha");

    // Closure that only *reads* the captured value
    let greet = || {
        println!("Hello, {}!", name);
    };

    // Call it multiple times
    greet();
    // Hello, Sasha!
    greet();
    // Hello, Sasha!
    greet();
    // Hello, Sasha!
}
