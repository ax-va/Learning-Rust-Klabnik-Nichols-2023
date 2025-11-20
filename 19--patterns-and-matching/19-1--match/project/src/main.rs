fn main() {

    /* match */

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
        _ => None, // no pattern for other cases with no binding a value with `_`
    };
    println!("y = {y:?}");
    // y = None

}
