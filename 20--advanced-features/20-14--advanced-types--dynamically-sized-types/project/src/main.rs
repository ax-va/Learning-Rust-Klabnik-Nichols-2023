/*
*Dynamically sized types* (DSTs) are types whose size is not known at compile time
and must therefore be used behind a pointer that carries size information at runtime.
 */

fn main() {
    // Example:
    // `str` is a dynamically sized type (DST) and represents a sequence of UTF-8 bytes.
    // The length of that sequence:
    // - is not known at compile time;
    // - varies at runtime.
    // So the compiler cannot know how large a plain str value would be.

    /*
    let s: str = "hello"; // error
     */

    let s: &str = "hello";

    // A reference to a sized type is a *thin pointer* that stores only the address,
    // while a `&str` is a *fat pointer* that stores both the address of the data and its length:
    // - a pointer to the first byte,
    // - the length of the string.

    // Even though `str` itself is a dynamically sized type (DST),
    // the compiler does know the size of `&str` at compile time.
    // Because that structure has a fixed size.
}
