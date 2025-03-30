/*
```
$ cd 07*
$ cd 07-02*
$ cargo new restaraunt --lib
$ cargo build
```
 */

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// nested module: restaurant's front of house
mod front_of_house {
    // Make the module public to its parent module
    // that means, if we can access `front_of_house`, we can access `hosting`.
    pub mod hosting {
        // Make the function public
        // that means, it is available if the module is public
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
/*
If module A is contained inside module B,
we say that module A is the *child* of module B and
that module B is the *parent* of module A.

module tree:

crate
|-- front_of_house
    |-- hosting
        |-- add_to_waitlist
        |-- seat_at_table
    |-- serving
        |-- take_order
        |-- serve_order
        |-- take_payment
```
 */

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // While `front_of_house` is not public,
    // because the `eat_at_restaurant` function is defined
    // in the same module as `front_of_house`
    // (that is, `eat_at_restaurant` and `front_of_house` are *siblings*),
    // we can refer to `front_of_house` from `eat_at_restaurant`.

    // Our preference in general is to specify absolute paths
    // because it is more likely we will want to move code definitions
    // and item calls independently of each other.

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
