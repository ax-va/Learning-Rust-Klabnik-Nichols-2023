/*
```
$ cargo test
```
 */

// This trait is the interface our mock object needs to implement
// so that the mock can be used in the same way a real object is.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(
        messenger: &'a T,
        max: usize
) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // The `set_value` method should be tested with the mock object
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You're at 75% of your quota!");
        }
    }
}

// The `#[cfg(test)]` attribute on the `tests` module tells Rust to compile
// and include the test code *only* when you run `cargo test`, not when you run `cargo build`.
// This means that using `cargo build` skips compiling test-related code,
// saving both compile time and space in the resulting binary.
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // This version of the mock will not work.
    /*

    // Implement a mock
    struct MockMessengerV1 {
        // Keep track of the messages it's told to send
        sent_messages: Vec<String>,
    }

    impl MockMessengerV1 {
        fn new() -> MockMessengerV1 {
            MockMessengerV1 {
                // Start with an empty list of messages
                sent_messages: vec![],
            }
        }
    }

    // Implement the `Messenger` trait for `MockMessengerV1`
    impl Messenger for MockMessengerV1 {
        fn send(&self, message: &str) {
            // Take the message passed in as a parameter and store it in the `MockMessengerV1` list of `sent_messages`
            self.sent_messages.push(String::from(message));
            // `Messenger::send(&self, ...)` takes `&self`
            // (that is a shorthand for `self: &Self`, which means the method takes an *immutable reference*),
            // but the mock tries to mutate internal state (`push` on a `Vec`).
            // As a result, `$ cargo test` which leads to
            // "error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference"
            // ```
            // self.sent_messages.push(String::from(message));
            // ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
            // ```
            // - We cannot modify the `MockMessengerV1` to keep track of the messages
            // because the send method takes an immutable reference to `self`.
            // - We also cannot take the suggestion from the error text to use `&mut self` instead
            // because then the signature of `send` would not match the signature in the `Messenger` trait definition.
        }
    }

     */

    // This version of mock uses `RefCell`, which works
    struct MockMessengerV2 {
        sent_messages: RefCell<Vec<String>>, // interior mutability
    }

    impl MockMessengerV2 {
        fn new() -> MockMessengerV2 {
            MockMessengerV2 {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessengerV2 {
        fn send(&self, message: &str) {
            // Mutate through `RefCell` even though we only have `&self`
            self.sent_messages.borrow_mut().push(String::from(message));
            // `RefCell` tracks how many outstanding
            // - immutable borrows (via `.borrow()` which yields `Ref<T>`)
            // - and mutable borrows (via `.borrow_mut()` which yields `RefMut<T>`)
            // are active at a given time.
            // It enforces the rule:
            // ---
            // You can have *any number* of immutable borrows
            // or *exactly one* mutable borrow,
            // but *not both* kinds at the same time.
            // ---

            // - `borrow()` gives us an *immutable* reference to the wrapped value inside the `RefCell<T>`.
            // So we can read things, call methods that require `&self`, etc.
            // But we cannot change the inner value via that reference.
            // - `borrow_mut()` gives us *exclusive mutable* access to the inner value.
            // So we can mutate the data like calling `push()` on a `Vec`.
            // While that mutable borrow is active,
            // we cannot have any other borrows (immutable or mutable).
            // If we try to violate these rules, `RefCell` will detect the violation at runtime and panic.

            // When we request `.borrow_mut()` or `.borrow()`,
            // `RefCell` checks the current state (how many borrows already exist).
            // If granting the new borrow would violate the borrowing invariant,
            // it doesn't compile-time reject
            // (because the compiler doesn't see inside `RefCell`'s internal state),
            // but instead panics at runtime.
            // `RefCell`'s API is designed to enforce safety this way.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        /*
        // Create a new `MockMessengerV1` that starts with an empty list of messages
        let mock_messenger_v1 = MockMessengerV1::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger_v1,
            100
        );

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger_v1.sent_messages.len(), 1);
        */

        let mock_messenger_v2 = MockMessengerV2::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger_v2,
            100
        );

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger_v2.sent_messages.borrow().len(), 1);
    }
}
