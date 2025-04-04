// Declare the child module as public to its parent module that means,
// if we can access the `front_of_house` module, we can access the `hosting` module too.
pub mod hosting;

// The module is private to its parent module by default that means,
// it is visible inside `front_of_house`, but invisible from outside `front_of_house`.
mod serving;

fn call_serving() {
    // The `serving` module is private to its parent `front_of_house` module,
    // but the child module is visible inside its parent module.
    serving::take_order();

    // However, we cannot access private functions of the child module:

    // compilation error: "error[E0603]: function `take_payment` is private"
    // serving::take_payment();
    //          ^^^^^^^^^^^^ private function
}
