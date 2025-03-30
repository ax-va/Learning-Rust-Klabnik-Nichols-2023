/*
```
$ cd 07*
$ cd 07-02*
$ cargo new restaraunt --lib
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

// restaurant's front of house
mod front_of_house {
    // If module A is contained inside module B,
    // we say that module A is the *child* of module B and
    // that module B is the *parent* of module A.

    mod hosting {
        fn add_to_waitlist() {/* ... */}

        fn seat_at_table() {/* ... */}
    }

    mod serving {
        fn take_order() {/* ... */}

        fn serve_order() {/* ... */}

        fn take_payment() {/* ... */}
    }
}