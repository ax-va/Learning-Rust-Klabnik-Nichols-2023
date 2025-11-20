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
        _ => None, // no pattern for other cases with no binding to a value with `_`
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


}
