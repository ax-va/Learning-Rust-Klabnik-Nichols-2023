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

    fn deref(&self) -> &Self::Target { // Returns `&T`, i.e. a reference to the wrapped `T`.
        // `.0` accesses the first value in a tuple struct.
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
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
    // Rust actually ran this code `*(a.deref())`.
    // `*` is because `a.deref()` returns a reference to the wrapped `i32`.
    assert_eq!(5, *a);

    // *Deref coercion* converts a reference to a type
    // that implements the `Deref` trait into a reference to another type.
    // It was added to Rust so that programmers writing function and method calls
    // don't need to add as many explicit references and dereferences with `&` and `*`.
    let m = MyBoxV2::new(String::from("Rust"));
    // `hello` expects a `&str`, but `&m` is `&MyBox<String>`.
    hello(&m);
    // Hello, Rust!

    // Rust applies deref coercion in steps
    // 1. `&MyBox<String>` to `&String` because of `impl<T> Deref for MyBox<T>`.
    // 2. `&String` to `&str` because of `impl Deref for String`.

    // Without deref coercion, an explicit equivalent would be
    hello(&(*m)[..]); // s[..] is a `str`.
    // Hello, Rust!

    // The number of times that `Deref::deref` needs to be inserted is resolved at compile time,
    // so there is no runtime penalty for taking advantage of deref coercion!

    // Rust does deref coercion in these three cases:
    // - from `&T` to `&U` when `T` implements `Deref` to some type `U`
    // - from `&mut T` to `&mut U` when `T` implements `DerefMut` to some type `U`
    // - from `&mut T` to `&U` when `T` implements `Deref` to some type `U`
    // But immutable references will never coerce to mutable references,
    // because it violates one of the borrowing rules.

    // Rust enforces at compile time that: we can have
    //  1. *any number of immutable references* (&T),
    // OR
    // 2. *exactly one mutable reference* (&mut T),
    // BUT
    // 3. never both at the same time.

    // This ensures:
    // - no data races,
    // - no undefined behavior from aliasing mutable and immutable pointers.
}
