/*
- *Strong references* (`Rc<T>`) share ownership.
  Cloning with `Rc::clone(&rc)` creates another owning handle and increments the strong count.

- *Weak references* (`Weak<T>`) do not own.
  `Rc::downgrade(&rc)` creates a `Weak<T>` and increments the weak count, not the strong count.
  A `Weak<T>` doesn't keep the value alive.

To access the value you call `upgrade() -> Option<Rc<T>>`.
You'll get a result of `Some` if the `Rc<T>` value has not been dropped yet
and a result of `None` if the `Rc<T>` value has been dropped.

- Drop vs deallocation
  - When `strong_count == 0`: the inner `T` is dropped.
  - When both `strong_count == 0` and `weak_count == 0`: the control block/allocation is freed.
  Therefore, weak pointers alone never prevent `T` from being dropped,
  but they can keep the control block around until they're gone.
 */

// Example: tree data structure

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]), // Add the reference to its children
    });
    // The `Node` in `leaf` now has two owners: `leaf` and ``branch`.
}
