use std::ops::Deref;

struct MyBoxV1<T>(T);
// - `struct` defines a custom data type in Rust.
// - `MyBoxV1<T>` is a generic tuple struct that takes a type parameter `T`.
// - The `(T)` means it holds exactly one value of type `T`.
// If you don't declare `<T>`, the compiler has no idea what `T` means.
// `T` by itself isn't a concrete type - it's a type parameter.

impl<T> MyBoxV1<T> {
    fn new(x: T) -> MyBoxV1<T> {
        MyBoxV1(x)
    }
}

struct MyBoxV2<T>(T);

impl<T> MyBoxV2<T> {
    fn new(x: T) -> MyBoxV2<T> {
        MyBoxV2(x)
    }
}

// The `Deref` trait, provided by the standard library,
// requires us to implement one method named `deref`
// that borrows `self` and returns a reference to the inner data.
impl<T> Deref for MyBoxV2<T> {
    type Target = T; // associated type required by the `Deref` trait

    fn deref(&self) -> &Self::Target {
        // `.0` accesses the first value in a tuple struct
        &self.0
    }
}

fn main() {
    // Implementing and using `new` is not strictly necessary.
    // We can already construct a value directly.
    let x = 5;
    let a = MyBoxV1(x);
    assert_eq!(5, x);
    // compilation error: "error[E0614]: type `MyBoxV1<{integer}>` cannot be dereferenced"
    // assert_eq!(5, *a);

    let x = 5;
    let a = MyBoxV1::new(x);
    assert_eq!(5, x);
    // compilation error: "error[E0614]: type `MyBoxV1<{integer}>` cannot be dereferenced"
    // assert_eq!(5, *a);

    let x = 5;
    let a = MyBoxV2::new(x);
    assert_eq!(5, x);
    // Behind the scenes, Rust actually ran this code `*(a.deref())`
    assert_eq!(5, *a);
}
