/*
A *union* is like a *struct*, but all fields share the same memory location.
Only one field is meant to be valid at a time.
For unions, writing is safe, reading is unsafe.

See also:
- https://doc.rust-lang.org/reference/items/unions.html
 */

union MyUnion {
    i: i32,
    f: f32,
}
// same 4 bytes in memory:
// - interpreted as i32
// - interpreted as f32

fn main() {
    let u = MyUnion { i: 0x3f800000 };

    // Reading a union field is always unsafe
    unsafe {
        println!("{}", u.i);
        // 1065353216
    }
    // same bits, different interpretation
    unsafe {
        println!("{}", u.f); // interpreting integer bits as float
        // 1
    }

    let mut u = MyUnion { i: 10 }; // safe
    // safe write
    u.i = 20;
    // unsafe read
    unsafe {
        println!("{}", u.i);
        // 20
    }
}
