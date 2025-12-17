fn main() {
    let mut num = 5;
    // `num` is a mutable `i32` stored somewhere in memory

    // Create raw pointers from references
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
    // `as *const i32` and `as *mut i32` cast those references into raw pointers

    // Notice:
    // you can have both an immutable and mutable raw pointer
    // to the same memory location, which Rust normally prevents with references,
    // because raw pointers are NOT checked by the borrow checker.

    // Dereferencing raw pointers requires `unsafe`
    unsafe {
        println!("r1 is: {}", *r1); // `*r1` means "read the value at the address stored in `r1`"
        // r1 is: 5
        println!("r2 is: {}", *r2); // same for `*r2`
        // r2 is: 5
    }
    // Rust requires an `unsafe` block because dereferencing a raw pointer
    // can easily be undefined behavior if the pointer isn't valid.

    // Create raw pointer to an arbitrary memory address
    let address = 0x012345usize;
    let r = address as *const i32;
    // Key points:
    // - This pointer is almost certainly invalid (not mapped memory, wrong alignment, not owned, etc.);
    // - Creating the pointer is allowed in safe Rust;
    // - But dereferencing it would require `unsafe` and would very likely crash or cause undefined behavior.
}
