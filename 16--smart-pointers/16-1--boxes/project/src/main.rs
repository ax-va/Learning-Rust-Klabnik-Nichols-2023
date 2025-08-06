fn main() {
    // Define the variable `b` to have the value of a `Box`
    // that points to the value 5,
    // which is allocated on the heap.
    let b = Box::new(5);
    println!("b = {b}");
    // b = 5

    // Use case: recursive types.
    // A value of a *recursive type* can have another value of the same type as part of itself.
} // The deallocation happens both for the box (stored on the stack)
// and the data it points to (stored on the heap).
