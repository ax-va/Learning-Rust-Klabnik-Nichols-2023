// The `Drop` trait is included in the prelude,
// so we don't need to bring it into scope.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping `CustomSmartPointer` with data '{}'!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("another stuff"),
    };
    println!("Instances of `CustomSmartPointer` created.");
    // Instances of `CustomSmartPointer` created.

    // Rust doesn't let you call the `Drop` trait's `drop` method manually.

    // compilation error: "error[E0040]: explicit use of destructor method"
    // e.drop();
    //   ^^^^ explicit destructor calls not allowed

    // But we can clean up it early by using `std::mem::drop`
    // that is already in the prelude.
    drop(e);
    // Dropping `CustomSmartPointer` with data 'another stuff'!

} // The instances of `CustomSmartPointer` go out of scope:
// Dropping `CustomSmartPointer` with data 'other stuff'!
// Dropping `CustomSmartPointer` with data 'my stuff'!

// Variables are dropped in the reverse order of their creation.
