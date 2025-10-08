use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // `RefCell<Rc<List>>` instead of `Rc<List>`
    Nil,
}
// `Rc<T>` = reference-counted pointer for shared ownership on the heap.
// `RefCell<T>` = interior mutability at runtime (borrow rules checked at runtime, not compile time).
// `List` is a simple cons list,
// but the "tail" pointer is `RefCell<Rc<List>>`,
// i.e. a mutable cell that holds a shared pointer.

impl List {
    // Access the second item if we have a `Cons` variant to mutate it through `borrow_mut()` later
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a's initial rc count = {}", Rc::strong_count(&a));
    // a's initial rc count = 1
    println!("a's next item = {:?}", a.tail());
    // a's next item = Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // `clone` clones the reference
    println!("a's rc count after b creation = {}", Rc::strong_count(&a));
    // a's rc count after b creation = 2
    println!("b's initial rc count = {}", Rc::strong_count(&b));
    // b's initial rc count = 1
    println!("b's next item = {:?}", b.tail());
    // b's next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    // Create a reference cycle:
    // 1. `a.tail()` returns `Some(&RefCell<Rc<List>>)`.
    // 2. Unwrap the Option: `link` is now a `&RefCell<Rc<List>>`.
    if let Some(link) = a.tail() {
        // 3. We have mutable access to the "tail" pointer of `a` through `borrow_mut()`.
        // 4. Dereference the reference to access the actual inner value that is currently `Rc::new(Nil)`.
        // 5. Replace that inner value with `Rc::clone(&b)`.
        // 6. `a`’s "tail" pointer, which previously pointed to `Nil`, now points to `b`.
        *link.borrow_mut() = Rc::clone(&b);
    }
    // Thus, `a` references `b` and `b` references `a`.

    println!("b's rc count after changing a = {}", Rc::strong_count(&b));
    // b's rc count after changing a = 2
    println!("a's rc count after changing a = {}", Rc::strong_count(&a));
    // a's rc count after changing a = 2

    // This will overflow the stack by the cycle:
    // it will recurse forever following the cycle and eventually stack-overflow.

    // println!("a's next item = {:?}", a.tail());

    // But this was not a memory leak yet, but just a stack-overflow.
} // The memory leak happens here:
// 1. Reference counts were 2 for both `a` and `b`.
// 2. When `main` ends, the stack variables `a` and `b` are dropped, reducing each count by 1.
// 3. Counts become 1 for both `a` and `b`.
// 4. Because neither `Rc`'s strong count reaches 0, their destructors never run.

// Thus, calling `Rc::clone` increases the `strong_count` of an `Rc<T>` instance,
// and an `Rc<T>` instance is only cleaned up if its `strong_count` is 0.

// When the program ends, the OS reclaims all of its memory, including any `Rc` cycles.
// There's no "system-wide" memory leak that persists after exit.

