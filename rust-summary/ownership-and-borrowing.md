# Ownership and borrowing

## Ownership 

*Ownership* is a set of rules that govern how a Rust program manages memory in the *heap*.
If any of the rules are violated, the program won't compile.

### Ownership rules

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable scope

This is an example of variable scope:
```rust
{
    // `s` is not valid here, since it's not yet declared

    let s = "hello"; // `s` is valid from this point forward

    // Do stuff with `s`
    
} // This scope is now over, and `s` is no longer valid
```

- When `s` comes *into* scope, it is valid.
- It remains valid until it goes *out of* scope.
- The allocated memory is automatically returned once the variable that owns it goes out of scope.
- Rust calls the `drop` function automatically at the closing curly bracket `}` to return the memory to the allocator.

### Ownership and functions

Passing a variable to a function will move or copy, just as assignment does.

### Return values and scope

```rust
fn main() {
    let s1 = gives_ownership(); // `gives_ownership` moves its return value into `s1`.
    let s2 = String::from("hello"); // `s2` comes into scope.
    let s3 = takes_and_gives_back(s2); // `s2` is moved into `takes_and_gives_back`,
    // which also moves its return value into `s3`.
} // Here, `s3` goes out of scope and is dropped.
// `s2` was moved, so nothing happens. `s1` goes out of scope and is dropped.

// This function will move its return value into the function that calls it.
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // `some_string` comes into scope.
    some_string
    // `some_string` is returned and moves out to the calling function.
}

// This function takes a `String` and returns a `String`.
fn takes_and_gives_back(a_string: String) -> String { // `a_string` comes into scope.
    a_string
    // `a_string` is returned and moves out to the calling function.
}
```

## Borrowing

### References

Creating a reference is called *borrowing*.

```rust
fn main() {
    let s1 = String::from("hello");
    // Use a function that uses a *reference* to an object as a parameter
    // instead of taking ownership of the value.
    let len = calculate_length(&s1); // Ampersands represent references.
    // `&s1` refers to the value of `s1` but does not own it.
    // The value it points to will not be dropped when the reference stops being used.
    println!("The length of '{s1}' is {len}.");
    // The length of 'hello' is 5.
    
    let s = String::from("hello");
    // If variables are immutable, their references are immutable too
    change(&s);
}

fn calculate_length(s: &String) -> usize { // The type of the parameter `s` is a reference.
    s.len() // Don't need to return the value in order to give back ownership.
} // Here, `s` goes out of scope. 
// But because it does not have ownership of what it refers to, the `String` is not dropped.

fn change(some_string: &String) {
    // compilation error: "error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference"
    // some_string.push_str(", world");
    // ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
```

- `s` points to `s1`:

| name     | value                    |
|----------|--------------------------|
| ptr      | <pointer to ptr of `s1`> |

- `s1` points to the data stored on the heap:

| name     | value                                |
|----------|--------------------------------------|
| ptr      | <pointer to the index 0 on the heap> |
| len      | 5                                    |
| capacity | 5                                    |

- data stored on the heap:

| index | value |
|-------|-------|
| 0     | `h`   |
| 1     | `e`   |
| 2     | `l`   |
| 3     | `l`   |
| 4     | `o`   |

### Mutable references

If you have a mutable reference to a value,
its use with other (mutable or immutable) references to that value is *restricted*.

The benefit of having this restriction is that
Rust can prevent *data races* at compile time that
happen when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There is no mechanism being used to synchronize access to the data.

### Dangling references not allowed

A *dangling pointer* is a pointer that references a location in memory that may have been given to someone else.
The memory can be freed up while preserving a pointer to that memory.
In Rust, the compiler guarantees that references will never be dangling references.

### The rules of references

- At any given time, you can have *either* one mutable reference or *any* number of immutable references.

- References must always be valid.

## Slices and string slices

*Slices* reference data starting at an including index and ending at an excluding index, 
for example, *string slices* 

```rust
// The `String` instance owns the data
let s = String::from("hello");
// Slices borrow parts of that data without ownership 
let slice = &s[0..2];
let slice = &s[..2];
let slice = &s[1..];
```
A slice is a kind of reference, so it does not have ownership.
Internally, the slice data structure stores the starting position and the length of the slice.

Notice:
String slice range indices must occur at valid UTF-8 character boundaries. 
If you attempt to create a string slice in the middle of a multibyte character, 
your program will exit with an error.
