use std::fmt; // the `fmt` format module within the `std` crate

// The `Display` trait is a *supertrait* for the `OutlinePrint` trait
// because `OutlinePrint` uses a method from `Display`.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // The `to_string` method is from the `Display` trait
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// The `Point` struct implements the `OutlinePrint` trait
impl OutlinePrint for Point {}

// The `Point` struct must also implement the `Display` trait
// because `OutlinePrint` relies on `Display`.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 1, y: 2 };

    point.outline_print();
    // **********
    // *        *
    // * (1, 2) *
    // *        *
    // **********
}
