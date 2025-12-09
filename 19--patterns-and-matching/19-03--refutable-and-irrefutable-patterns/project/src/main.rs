/*
Refutable and irrefutable patterns:

- *Irrefutable pattern* always matches.
Used in `let`, function parameters, and `for`.
Example:
```
let (x, y) = (1, 2);     // always matches
```

- *Refutable pattern* might fail to match.
Used in `if let`, `while let`, and `match` arms.
Example:
```
if let Some(v) = opt {   // may or may not match
    ...
}
```
 */

fn main() {
    // Try to use a refutable pattern
    // where an irrefutable pattern is required
    // and vice versa.

    // compilation error: "error[E0005]: refutable pattern in local binding"
    // let Some(x) = Some(5);
    //     ^^^^^^^ pattern `None` not covered

    // Fix this
    if let Some(x) = Some(5) {
        println!("{x}");
        // 5
    }

    // The compiler will give a warning: "warning: irrefutable `if let` pattern"
    if let x = 6 {
        println!("{x}");
        // 6
    }

}
