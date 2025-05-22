/*
```
$ cd 10*
$ cd 10-7*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() { // outer scope
    // We declare variables without giving them an initial value,
    // so the variable name exists in the outer scope.
    let r;

    { // inner scope
        let x = 5;
        // We attempt to set the value of `r` as a reference to `x`.
        // So we attempt to use a reference whose value has gone out of scope.
        
        // compilation error: "error[E0597]: `x` does not live long enough"
        r = &x;
        //  ^^ borrowed value does not live long enough
    }
    // - `x` dropped here while still borrowed

    // When we pass `r` to `println!`,
    // we're passing a copy of the reference.
    println!("r: {r}");
    //           --- borrow later used here
}
// Explanation:
// The value that `r` is referring to
// has gone out of scope before we try to use it, i.e.,
// `x` will be out of scope when the inner scope ends.
// But `r` is still valid for the outer scope
// because its scope is larger, `r` "lives longer".
// When the inner scope ends, the memory allocated for `x` is deallocated,
// and `r` would become a *dangling reference*.
// The Rust compiler has a *borrow checker*
// that compares scopes to determine whether all borrows are valid.
