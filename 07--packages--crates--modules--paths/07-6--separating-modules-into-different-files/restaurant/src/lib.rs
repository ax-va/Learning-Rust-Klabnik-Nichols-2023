/*
Module tree:

crate root file // root module in the library crate
|-- front_of_house // module
    |-- hosting // public module to its parent
        |-- add_to_waitlist // public function
        |-- seat_at_table // function
    |-- serving // module
        |-- take_order // public function
        |-- serve_order // function
        |-- take_payment // function
    |-- call_serving // function
|-- back_of_house // module
    |-- Breakfast // public struct
        |-- toast // public field
        |-- seasonal_fruit // field
        |-- summer // public constructor
    |-- Appetizer // public enum with default-public variants
        |-- Soup // public variant
        |-- Salad // public variant
    |-- cook_order // function
    |-- fix_incorrect_order // function
|-- deliver_order // function
|-- eat_at_restaurant // public function
|-- ...

```
$ cd 07*
$ cd 07-02*
$ cargo new restaurant --lib
$ cargo build
```
 */

// created automatically for the library crate
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// created automatically for the library crate
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Add contents to the `crate` module

// Declare modules to load them to file
mod front_of_house;
mod back_of_house;
// Notice:
// Load a file using a `mod` declaration *once* in the module tree.
// Then, refer to the code of loaded files using a path to where it was declared.
// In other words, `mod` is *not* an `include` or `import` operations as in other programming languages.

// *Re-export* the public module into the scope making it also public in this scope.
pub use crate::front_of_house::hosting;

fn deliver_order() {/* ... */}

// public API
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // clear

    // Create mutable struct.
    // We can access the struct because it is public.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Access public field of struct
    println!("meal.toast: {}", meal.toast);
    // Write to public field because struct is mutable
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Access variants of public enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
