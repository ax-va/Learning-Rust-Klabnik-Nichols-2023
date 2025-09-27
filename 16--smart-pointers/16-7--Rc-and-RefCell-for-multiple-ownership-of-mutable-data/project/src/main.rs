// `Rc<T>` lets us have multiple owners of some data,
// but it only gives immutable access to that data.
// If we have an `Rc<T>` that holds a `RefCell<T>`, we can get a value
// that can have multiple owners and that you can mutate.

#[derive(Debug)]
enum List {
    Cons(
        Rc<RefCell<i32>>, // head node = shared (`Rc`) and mutably borrowable at runtime (`RefCell`) value
        Rc<List>  // tail list = shared pointer to the rest of the list
    ),
    Nil, // empty list
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // `value` is a shared, mutable-internally integer: `Rc<RefCell<i32>>` initially 5
    let value = Rc::new(RefCell::new(5));
    // - `value` owns an Rc<RefCell<i32>>.
    // - The `Rc` keeps a reference count. Initially, `strong_count` = 1.
    // - Inside the `Rc`, the `RefCell<i32>` owns the `i32`.

    // `a` is an `Rc<List>` pointing to a single-node list
    let a = Rc::new(
        Cons(Rc::clone(&value), // Clones the reference
        Rc::new(Nil))
    );
    // `value` and `a` share ownership of the integer 5.

    // `a` is the shared tail.
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("value after = {:?}", value);
    // value after = RefCell { value: 15 }
    println!("a after = {:?}", a);
    // a after = Cons(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b);
    // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c);
    // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}
