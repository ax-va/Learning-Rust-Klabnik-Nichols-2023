/*
Consider generics in

- function definitions
- struct definitions
- enum definitions
- methods definitions

```
$ cd 10*
$ cd 10-01*
$ cargo new project
$ cd project
$ cargo run
```
 */

/* generics in function definitions */

// Explanation for the `max` function:
// - The function is generic over some type `T`;
// - The function has one parameter, which is a slice of values of type `T`;
// - The function will return a reference to a value of the same type `T`;
// - The types valid for `T` are restricted to be compared in the function.
fn max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }

    max
}

/* generics in struct definitions */

struct PointV1<T> {
    x: T,
    y: T,
}

/* multiple generic type parameters */

struct PointV2<T, U> {
    x: T,
    y: U,
}

/*

generics in enum definitions

```
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

*/


/* generics in method definitions */

// By declaring `T` as a generic type after `impl`,
// Rust can identify that the type in the angle brackets in `Point`
// is a generic type rather than a concrete type.
impl<T> PointV1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Other instances of `Point<T>`
// where `T` is not of type `f32`
// will not have this method defined.
impl PointV1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// mix up types
struct PointV3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointV3<X1, Y1> {
    fn mixup<X2, Y2> (
        self,
        other: PointV3<X2, Y2>,
    ) -> PointV3<X1, Y2> {
        PointV3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65, 11];
    let result = max(&number_list);
    println!("The maximum number is {result}");
    // The maximum number is 100

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = max(&char_list);
    println!("The maximum char is {result}");
    // The maximum char is y

    let integer_point = PointV1 { x: 5, y: 10 };
    let float_point = PointV1 { x: 1.0, y: 4.0 };

    // compilation error: "error[E0308]: mismatched types"
    // let wont_work = PointV1 { x: 5, y: 4.0 };
    //                                    ^^^ expected integer, found floating-point number

    // Explanation:
    // The compiler inferred the integer type for `T` from 5.

    let integer_and_float_point = PointV2 { x: 5, y: 4.0 };

    let p = PointV1 { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // p.x = 5

    let p = PointV1 { x: 5.0, y: 10.0 };
    println!("p.distance_from_origin() = {}", p.distance_from_origin());
    // p.distance_from_origin() = 11.18034

    // mix up types
    let p1 = PointV3 { x: 5, y: 10.4 };
    let p2 = PointV3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // p3.x = 5, p3.y = c

}
