#[no_mangle]  // Keeps function name exactly "rust_add"
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    println!("Calling `rust_add` with: {}, {}", a, b);
    a + b
}

// 1. Build:
/*
$ cargo build --release
 */

// 2. Now we have got 'libproject.so' in '.../project/target/release/'.

// 3. Compile and link with the Rust library
/*
$ gcc src/main.c -L target/release -lproject -o main
$ LD_LIBRARY_PATH=target/release ./main
Add in Rust: 2 + 3
Result: 5
 */