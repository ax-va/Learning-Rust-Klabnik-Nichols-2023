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

// Declare enum as public to make all its variants public
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
