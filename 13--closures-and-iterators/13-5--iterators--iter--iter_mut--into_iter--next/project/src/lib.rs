/*
Run tests
$ cargo test
 */

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    // Calling the `next` method on an iterator changes internal state
    // that the iterator uses to keep track of where it is in the sequence,
    // so we need to make the iterator variable mutable.
    let mut v1_iter = v1.iter();
    // Notice:
    // We didn't need to make `v1_iter` mutable when we used a `for` loop
    // because the loop took ownership of `v1_iter` and made it mutable behind the scenes.

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // `next` does not consume the iterator itself - it only mutably borrows it to advance its internal state
    assert_eq!(v1_iter.next(), None);
}