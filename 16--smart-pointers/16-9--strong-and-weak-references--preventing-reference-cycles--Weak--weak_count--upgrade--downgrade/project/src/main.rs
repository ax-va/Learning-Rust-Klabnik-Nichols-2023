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
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // The child node does not own its parent node, but has a reference to it.
    children: RefCell<Vec<Rc<Node>>>, // The parent node owns its child nodes.
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "1 - leaf: strong_count = {}, weak_count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // 1 - leaf: strong_count = 1, weak_count = 0

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), // Add the reference to its children.
    }); // The `Node` in `leaf` now has two owners: `leaf` and ``branch`.

    println!(
        "2 - branch: strong_count = {}, weak_count = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );
    // 2 - branch: strong_count = 1, weak_count = 0

    println!(
        "3 - leaf: strong_count = {}, weak_count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // 3 - leaf: strong_count = 2, weak_count = 0

    // Modify `leaf` to give it a `Weak<Node>` reference to its parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "4 - branch: strong_count = {}, weak_count = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );
    // 4 - branch: strong_count = 1, weak_count = 1

    println!(
        "5 - leaf: strong_count = {}, weak_count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // 5 - leaf: strong_count = 2, weak_count = 0

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })

    println!(
        "6 - leaf: strong_count = {}, weak_count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // 6 - leaf: strong_count = 2, weak_count = 0
}
// Dropping `branch` (created last):
//  - `branch`'s `Rc` strong count: 1 -> 0 -> its `Node { value: 5, ... }` is dropped.
//  - While dropping that node, its children: `Vec<Rc<Node>>` is dropped.
//  - This decreases `leaf`'s strong count: 2 -> 1.
//  - `branch`'s value is gone, but the control block stays alive until the weak count hits 0.
//  - `branch` still has a weak ref outstanding, so: `branch`: strong = 0, weak = 1.
// - `leaf.parent.borrow().upgrade()` would now return `None`.

// Dropping `leaf` (created first):
// - `leaf`'s `Rc` strong count: 1 -> 0 -> its `Node { value: 3, ... }` is dropped.
// - While dropping that node, its parent `Weak<Node>` (which points to `branch`) is dropped.
// - This decreases `branch`'s weak count: 1 -> 0.
// - Since `branch` already had strong = 0, its control block is now freed.
// - `leaf` has no weak refs, so after its value drops, its own control block is freed immediately.
