fn main() {

    /* ------- */
    /* `match` */
    /* ------- */

    // Notice:
    // The compiler checks for exhaustiveness

    // let x: Option<i32> = Some(5);
    // The type `Option<i32>` can be inferred
    let x = Some(5);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("y = {y:?}");
    // y = Some(6)

    let x: Option<i32> = None;
    let y = match x {
        Some(i) => Some(i + 1),
        _ => None, // `_` means no pattern for other cases and no binding to a variable
    };
    println!("y = {y:?}");
    // y = None

    /* ---------------------------------- */
    /* `if let`, `if else`, `if else let` */
    /* ---------------------------------- */

    // Notice:
    // The compiler does not check for exhaustiveness.
    // This can lead to possible logic bugs.

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // Rust doesn't require that the conditions in a series of
    // `if let`, `else if`, and `else if let` arms relate to each other.
    if let Some(color) = favorite_color {
        // `color` is a new shadowed variable in this scope
        println!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // `age` is a new shadowed variable in this scope
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    // Using purple as the background color

    /* ----------- */
    /* `while let` */
    /* ----------- */

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // The `while let` loop continues
    // running the code in its block
    // as long as `pop` returns `Some`.
    // When `pop` returns `None`, the loop stops.
    while let Some(top) = stack.pop() {
        println!("top = {top}");
    }
    // top = 3
    // top = 2
    // top = 1

    /* ----- */
    /* `for` */
    /* ----- */

    let v = vec!['a', 'b', 'c'];
    // Use a pattern in the `for` loop to destructure tuples
    for (index, value) in v.iter().enumerate() {
        println!("index = {index}, value = {value}");
    }
    // index = 0, value = a
    // index = 1, value = b
    // index = 2, value = c

    /* ----- */
    /* `let` */
    /* ----- */

    let (x, y, z) = (1, 2, 3);
    println!("x = {x}, y = {y}, z = {z}");
    // x = 1, y = 2, z = 3

    // compilation error: "error[E0308]: mismatched types"
    // let (x, y) = (1, 2, 3);
    //     ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})
    //     |
    //     expected a tuple with 3 elements, found one with 2 elements

    // Ignore one or more of the values in the tuple using `..`
    let (x, y, ..) = (1, 2, 3, 4);
    println!("x = {x}, y = {y}");
    // x = 1, y = 2
}
