/*
The compiler cannot verify the safety guarantees of an *unsafe trait*.
The person implementing it must ensure certain invariants manually.

Mental model:
`unsafe fn`     ->  dangerous to call
`unsafe trait`  ->  dangerous to implement
`unsafe impl`   ->  you're promising invariants are true
 */

// Unsafe trait: implementer must guarantee correctness
unsafe trait TrustedLength {
    // Must return the exact number of initialized elements
    fn len(&self) -> usize;

    // Pointer to the first element
    fn as_ptr(&self) -> *const i32;
}

// A simple container
struct MyVec {
    data: Vec<i32>,
}

// Correct implementation
unsafe impl TrustedLength for MyVec {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_ptr(&self) -> *const i32 {
        self.data.as_ptr()
    }
}

// Broken container
struct BrokenVec {
    data: Vec<i32>,
}

// WRONG implementation
unsafe impl TrustedLength for BrokenVec {
    fn len(&self) -> usize {
        // Lie about how many elements exist
        1_000
    }

    fn as_ptr(&self) -> *const i32 {
        self.data.as_ptr()
    }
}

// Function relying on the trait's guarantee
fn sum_all<T: TrustedLength>(container: &T) -> i32 {
    let len = container.len();
    let ptr = container.as_ptr();
    let mut sum = 0;

    unsafe {
        for i in 0..len {
            // Reads memory assuming it's initialized
            sum += *ptr.add(i);
        }
    }

    sum
}

fn main() {
    let good = MyVec {
        data: vec![1, 2, 3],
    };

    println!("Correct sum: {}", sum_all(&good));
    // Correct sum: 6

    let bad = BrokenVec {
        data: vec![1, 2, 3],
    };

    // WARNING:
    // this may read invalid memory
    // because `BrokenVec` violates the `TrustedLength` safety contract.
    // println!("Broken sum: {}", sum_all(&bad));

    // This compiles.
    // But it may:
    // - read garbage memory
    // - crash
    // - produce random results
}
