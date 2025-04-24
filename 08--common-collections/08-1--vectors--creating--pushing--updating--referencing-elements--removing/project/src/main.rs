/*
```
$ cd 08*
$ cd 08-1*
$ cargo new project
$ cd project
$ cargo run
```
 */

fn main() {
    // Create an immutable empty vector
    let v: Vec<i32> = Vec::new();

    // Create an immutable `Vec<i32>` by using macro
    let v = vec![0, 1, 2, 3];

    // Create a mutable vector and then add elements to it
    let mut v = Vec::new();

    // using `push`

    v.push(0); // Rust has inferred the `i32` type
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(2_147_483_647);

    // Try to push a value out the range of `i32`:
    // compilation error: "error: literal out of range for `i32`"
    // v.push(2_147_483_648);
    //        ^^^^^^^^^^^^^

    // Change the first element of the vector from 0 to -2_147_483_648
    v[0] = -2_147_483_648;

    // There are two ways to reference a value stored in a vector:

    // 1. via indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // 2. by using the `get` method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // Causes the program to *panic*; it stops here
    // Output: "index out of bounds: the len is 5 but the index is 100"

    let does_not_exist = v.get(100); // Returns `None` without panicking
    match does_not_exist {
        Some(does_not_exist) => println!("The third element is {does_not_exist}"),
        None => println!("There is no third element."),
    }
    // There is no third element.

    // We cannot mix mutable and immutable references in the same scope
    let mut v = vec![1, 2, 3, 4];
    let first = &v[0]; // Immutable borrow occurs here
    // compilation error: "error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable"
    // v.push(6);
    // ^^^^^^^^^ mutable borrow occurs here
    println!("The first element is: {first}");

    /*
    Explanation:
    Vectors put the values next to each other in memory.
    If there is not enough allocated memory space,
    adding a new element onto the end of the vector might require re-allocating new memory
    to copy the elements to the new space.
    In that case, the reference to the first element would be pointing to deallocated memory.
     */

    // How to fix it:

    // solution 1: *immutable borrow* only within a block
    let mut v = vec![1, 2, 3, 4];
    {
        let first = &v[0];
        println!("First: {}", first);
        // First: 1
    } // Immutable borrow ends here

    v[0] = 0; // Now OK to mutate
    println!("First: {}", v[0]);
    // First: 0

    // solution 2: *mutable borrow* to change the value through mutable reference
    let mut v = vec![1, 2, 3, 4];
    let first = &mut v[0];
    // Dereference the reference with `*` to access to the actual `i32` value it points to
    *first = 0;
    println!("First: {}", first);
    // First: 0

    // using `pop`

    let mut v = vec![1, 2];

    let removed = v.pop(); // type: `Option<{integer}>`
    match removed {
        Some(removed) => println!("Removed: {removed}"),
        None => println!("No elements in vector."),
    }
    // Removed: 2

    let removed = v.pop(); // type: `Option<{integer}>`
    match removed {
        Some(removed) => println!("Removed: {removed}"),
        None => println!("No elements in vector."),
    }
    // Removed: 1

    let removed = v.pop(); // type: `Option<{integer}>`
    match removed {
        Some(removed) => println!("Removed: {removed}"),
        None => println!("No elements in vector."),
    }
    // No elements in vector.
}
