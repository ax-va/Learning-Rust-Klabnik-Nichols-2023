fn main() {
    // This Rust code shows how to safely obtain
    // two mutable references to different parts of the same collection,
    // something that is normally forbidden by the borrow checker
    // unless Rust can prove it's safe.

    // It safely splits a mutable slice
    // into two non-overlapping mutable slices using `split_at_mut`,
    // which relies on internal unsafe code to uphold Rust's borrowing guarantees.

    // Create a vector
    let mut v = vec![10, 20, 30, 40, 50, 60];
    // `v` is a growable `Vec<i32>` allocated on the heap

    // We need a mutable slice.
    // `split_at_mut` takes exclusive access to the data
    // in order to safely produce two non-overlapping mutable slices.

    // Take a mutable slice of the whole vector
    let r = &mut v[..];
    // `&mut v[..]` converts the vector into a mutable slice: `&mut [i32]`.
    // This borrows the *entire vector contents* mutably.
    // While `r` exists, `v` itself cannot be accessed.

    // Split the mutable slice: return two mutable slices
    let (a, b) = r.split_at_mut(3);
    // Normally, this is not allowed:
    /*
    let a = &mut v[0..3];
    let b = &mut v[3..6]; // <- borrow checker error
     */

    // Borrow checker rule (normally):
    // **Only one `&mut` reference at a time.**
    // Rust's borrow checker can't understand
    // that we're borrowing different parts of the slice;
    // it only knows that we're borrowing from the same slice twice.

    // But `split_at_mut`:
    // - is is a *safe* function to call;
    // - is implemented using *unsafe* internally;
    // - guarantees the two slices are non-overlapping;
    // - encodes that guarantee in its API.
    // So the borrow checker allows it.

    // Internally, `split_at_mut`:
    // - takes a raw pointer to the slice;
    // - creates two raw pointers at different offsets;
    // - converts them back into `&mut [T]`;
    // - ensures the slices don't overlap.
    // All wrapped in a safe API so you don't need `unsafe`.

    assert_eq!(a, &mut [10, 20, 30]);
    assert_eq!(b, &mut [40, 50, 60]);

    // We can mutate both parts independently:
    a[0] += 1;
    b[0] += 1;
    println!("{}", a[0]);
    // 11

    assert_eq!(v, vec![11, 20, 30, 41, 50, 60]);
    // We can use `v` there
    // because `r`, `a`, and `b`
    // are no longer used after that point,
    // so Rust ends their mutable borrows early
    // (thanks to *Non-Lexical Lifetimes*).
}
