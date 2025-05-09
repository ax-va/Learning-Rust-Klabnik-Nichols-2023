/*
```
$ cd 09*
$ cd 09-6*
$ cargo new project
$ cd project
$ cargo run
```
 */

pub struct Guess {
    value: i32,
}

impl Guess {
    // This constructor creates instances of `Guess` values.
    // Code outside the module *must* use the `Guess::new` function to create a `Guess`
    // because the `value` field is private.
    // Thus, we *ensure* there is no way for a ´Guess` to have a value
    // that has not been checked by the conditions in the `Guess::new` function.
    pub fn new(value: i32) -> Guess {
        // Check the condition
        if value < 1 || value > 100 {
            // Creating a `Guess` with a `value` outside this range
            // violates the contract that `Guess::new` is relying on.
            // The conditions in which `Guess::new` panics should be
            // discussed in its public-facing API documentation.
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    // This getter borrows `self` returns a value of its `value` field.
    // This public method is necessary because the `value` field is private.
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Hello, world!");
}
