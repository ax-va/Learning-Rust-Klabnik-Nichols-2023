fn main() {
    // Define the variable `b` to have the value of a `Box`
    // that points to the value 5,
    // which is allocated on the heap.
    let b = Box::new(5);
    println!("b = {b}");
    // b = 5

    let x = 5;
    // Make a reference to `x`
    let y = &x;
    // Use an instance of a box pointing to a copied value of `x`
    // rather than a reference pointing to the value of `x`.
    let z = Box::new(x);

    assert_eq!(5, x);
    // Use the dereference operator `*`
    assert_eq!(5, *y);
    // compilation error: "error[E0277]: can't compare `{integer}` with `&{integer}`"
    // assert_eq!(5, y);
    assert_eq!(5, *z);

} // The deallocation happens both for the box (stored on the stack)distraction.
// and the data it points to (stored on the heap).
