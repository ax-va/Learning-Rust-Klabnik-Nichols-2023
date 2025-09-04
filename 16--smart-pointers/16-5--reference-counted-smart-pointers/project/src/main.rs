// It doesn't work
enum ListV1 {
    ConsV1(i32, Box<ListV1>),
    NilV1,
}

// It works.
// Each `Rc<T>` keeps track of how many owners (pointers) exist for the same underlying value.
enum ListV2 {
    ConsV2(i32, Rc<ListV2>),
    NilV2,
}

use crate::ListV1::{ConsV1, NilV1};
use crate::ListV2::{ConsV2, NilV2};
// `Rc<T>` is not in the prelude, so we need to bring it into scope
use std::rc::Rc;

fn main() {
    // compilation error: "error[E0382]: use of moved value: `a`"
    // let a = ConsV1(5, Box::new(ConsV1(10, Box::new(NilV1))));
    //     - move occurs because `a` has type `ListV1`, which does not implement the `Copy` trait
    // let b = ConsV1(3, Box::new(a));
    //     - value moved here
    // let c = ConsV1(4, Box::new(a));
    //     ^ value used here after move

    // Explanation:
    // When we create `b`, `a` is moved into `b` and `b` owns `a`.
    // Then, when we try to use `a` again when creating `c`,
    // we're not allowed to because `a` has been moved.

    let a = Rc::new(ConsV2(5, Rc::new(ConsV2(10, Rc::new(NilV2)))));
    let b = ConsV2(3, Rc::clone(&a));  // We're cloning the *reference*, not the underlying list.
    let c = ConsV2(4, Rc::clone(&a));

    // When we call `Rc::clone(&a)`, it:
    // 1. Increments the reference count.
    // 2. Returns a new `Rc<T>` pointing to the same heap allocation.
    // So both the old `Rc` and the cloned one refer to the same data, not copies.

    // We could have called `a.clone()` rather than `Rc::clone(&a)`,
    // but Rust's convention is to use `Rc::clone` in this case.
    // By using `Rc::clone` explicitly, we can idiomatically distinguish
    // between clones that perform a deep copy of the underlying data
    // and clones that simply increase the reference count and share ownership.
}
