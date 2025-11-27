fn main() {
    /* ----------------- */
    /* matching literals */
    /* ----------------- */

    let x = 1;
    match x {
        // integer literal patterns
        // that the value `x` is compared against
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // one

    /* ------------------------ */
    /* matching named variables */
    /* ------------------------ */

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // Introduce a new variable named `y`
        // that will match any value inside a `Some` value
        // and shadow `y` from the outer scope.
        // This new `y` binds to the inner value of the `Some` in `x`.
        Some(y) => println!("Matched, y = {y}"),
        // `x` from the outer scope
        _ => println!("Default case, x = {:?}", x),
    }
    // Matched, y = 5
    println!("At the end: x = {:?}, y = {y}", x);
    // At the end: x = Some(5), y = 10

    /* ----------------- */
    /* multiple patterns */
    /* ----------------- */

    let x = 1;
    match x {
        // 1 or 2
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // one or two

    /* ------------------------------------ */
    /* matching ranges of values with `..=` */
    /* ------------------------------------ */

    // Ranges are only allowed with numeric or char values

    let x = 3;
    match x {
        1..=5 => println!("In range [1, 5]"),
        _ => println!("Not in range [1, 5]"),
    }
    // In range [1, 5]

    let x = 'c';
    match x {
        'a'..='g' => println!("Within ASCII characters: 'a' through 'g'"),
        'j'..='z' => println!("Within ASCII characters: 'j' through 'z'"),
         _ => println!("something else"),
    }
    // Within ASCII characters: 'a' through 'g'
}
