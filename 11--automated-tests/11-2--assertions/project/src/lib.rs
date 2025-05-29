/*
```
$ cd 11*
$ cd 11-2*
$ cargo new project --lib
$ cd project
```

Run all tests in the project

```
$ cargo test
```
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self /* short for `self: &Self` */, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    // Because the tests module is an inner module,
    // we need to bring the code under test into the scope of the inner module.
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // If the test fails, `assert_eq!` displays the values of
        // the left and right arguments that were compared.
        assert_eq!(4, add_two(2));
        // `assert_ne!` is used similarly.
        // The values being compared must implement the `PartialEq` and `Debug` traits
        // to assert equality of those types and to print the values when the assertion fails, respectively.
        // Use `#[derive(PartialEq, Debug)]` in Rust to automatically generate implementations of
        // the `PartialEq` and `Debug` traits for a custom type (usually a struct or enum),
        // which are commonly needed in practice, especially in testing and comparison.
        /*
        ```
        #[derive(PartialEq, Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };
        assert_eq!(a, b); // Works because `PartialEq` is derived
        ```
         */
    }
}
