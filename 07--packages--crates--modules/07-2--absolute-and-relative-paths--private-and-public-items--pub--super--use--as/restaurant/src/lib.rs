/*
Module tree:

crate root file // root module in the library crate
|-- deliver_order // function
|-- eat_at_restaurant // public function
|-- front_of_house // module
    |-- hosting // public module to its parent
        |-- add_to_waitlist // public function
        |-- seat_at_table // function
    |-- serving // module
        |-- take_order // function
        |-- serve_order // function
        |-- take_payment // function
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

// Bring the `crate::front_of_house::hosting` module
// into the scope ot the library crate.
// This is *clear*.
/*
`use crate::front_of_house::hosting;
 */
// The name `hosting` available in the new scope is private.

// *Re-export* the public module into the scope making it also public in this scope.
pub use crate::front_of_house::hosting;

// Bring the `crate::front_of_house::hosting::add_to_waitlist` function
// into the scope ot the library crate.
// This is *unclear*.
use crate::front_of_house::hosting::add_to_waitlist;

// But when bringing structs, enums, and other items into scope,
// the convention is to use the full path for them, for example:
/*
use std::collections::HashMap;
 */

// The exception to this idiom is when two items have the same name.
// 1. The one idiomatic way is to use parent modules:
/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
// ...
}

fn function2() -> io::Result<()> {
// ...
}
 */

// The other idiomatic way is to use the `as` keyword:
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
// ...
}

fn function2() -> IoResult<()> {
// ...
}
 */
// The both ways are conventional.

fn deliver_order() {/* ... */}

// public API
pub fn eat_at_restaurant() {
    // 1. *absolute path*
    crate::front_of_house::hosting::add_to_waitlist();
    // While `front_of_house` is not public,
    // because the `eat_at_restaurant` function is defined
    // in the same module as `front_of_house`
    // (that is, `eat_at_restaurant` and `front_of_house` are *siblings*),
    // we can refer to `front_of_house` from `eat_at_restaurant`.

    // 2. *relative path*
    front_of_house::hosting::add_to_waitlist();

    // 3. by using the `use` keyword to bring the module into the scope
    hosting::add_to_waitlist(); // clear
    // The `eat_at_restaurant` function can see the `hosting` module
    // because they are in the same scope of the current module.
    // If we move `eat_at_restaurant` into a child module,
    // we should add `super` the `super::hosting::add_to_waitlist();`.

    // 4. by using the `use` keyword to bring the function into the scope
    add_to_waitlist(); // unclear

    // Create mutable struct.
    // We can access the struct because it is public.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Access public field of struct
    println!("meal.toast: {}", meal.toast);
    // Write to public field because struct is mutable
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // We cannot access private fields:

    // compilation error: "error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private"
    // println!("meal.seasonal_fruit: {}", meal.seasonal_fruit);
    //                                          ^^^^^^^^^^^^^^ private field

    // Access variants of public enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// nested module: restaurant's front of house
mod front_of_house {
    // Make the child module public to its parent module that means,
    // if we can access the `front_of_house` module, we can access the `hosting` module too.
    pub mod hosting {
        // Make the function public that means,
        // the function is available if the module is public.
        pub fn add_to_waitlist() {/* ... */}

        fn seat_at_table() {/* ... */}
    }

    // The module is private to its parent module by default
    mod serving {
        fn take_order() {/* ... */}

        fn serve_order() {/* ... */}

        fn take_payment() {/* ... */}
    }
}

// restaurant's back of house
mod back_of_house {
    // public struct
    pub struct Breakfast {
        // public field
        pub toast: String,
        // private field
        seasonal_fruit: String,
    }

    impl Breakfast {
        // public constructor
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Public enum makes all its variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn cook_order() {/* ... */}

    fn fix_incorrect_order() {
        // Call the sibling function
        cook_order();
        // Use *relative path with `super`* to reference
        // an item in its parent module, which is `crate` (the root) in this case.
        super::deliver_order();
        // This makes rearranging the module tree easier
        // when the module is closely related to the parent
        // but the parent might be moved elsewhere in the module tree.
    }
}
