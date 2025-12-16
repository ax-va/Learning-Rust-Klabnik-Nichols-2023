/*
A *match guard* is an additional `if` condition, specified after the pattern in a match arm.
 */

fn main() {
    let num = Some(4);

    match num {
        // Here, `x` would shadow an outer `x`
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    // The number 4 is even

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // `y` in `Some(y)` would shadow the outer `y`
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    // Default case, x = Some(5)
    println!("After match: x = {:?}, y = {y}", x);
    // After match: x = Some(5), y = 10

    let x = 4;
    let y = false;

    match x {
        // The match guard will apply to
        // the whole pattern `4 | 5 | 6 `,
        // not just to the last value 6,
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    // no
}
