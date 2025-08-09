fn main() {
    // Define the variable `b` to have the value of a `Box`
    // that points to the value 5,
    // which is allocated on the heap.
    let b = Box::new(5);
    println!("b = {b}");
    // b = 5
} // The deallocation happens both for the box (stored on the stack)distraction.
// and the data it points to (stored on the heap).
