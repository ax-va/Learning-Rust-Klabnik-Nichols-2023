fn main() {
    // immutable value
    let x = 5;

    // Try to borrow the immutable value as mutable:
    // compilation error: "error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable"
    // let y = &mut x;
    //         ^^^^^^ cannot borrow as mutable
}
