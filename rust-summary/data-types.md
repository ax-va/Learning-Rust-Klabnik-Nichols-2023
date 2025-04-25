# Stack-allocated data types

Rust is a *statically typed* language, which means that it must know the types of all variables at compile time.
Next types are of a known size, i.e. they can be stored on the stack and popped off the stack 
when their scope is over, and can be quickly and trivially copied to make a new, independent instance 
if another part of code needs to use the same value in a different scope.

## Scalar types

A *scalar* type represents a single value.
Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

### Integers

Signed variants can store numbers in the range `[-2^(n - 1), 2^(n - 1) - 1]`, 
while unsigned variants can store numbers in the range `[0,  2^n - 1]`.
The `isize` and `usize` types depend on the architecture of the computer a program is running on, 
which is denoted in the table as "arch": 64 bits for a 64-bit architecture and 32 bits for a 32-bit architecture.
Integer types default to `i32`.

| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |  
| 128-bit | `i128`  | `u128`   |  
| arch    | `isize` | `usize`  |

#### Integer overflow

In debug mode, Rust includes checks for integer overflow that
cause your program to *panic* at runtime if this behavior occurs.

In release mode with the `--release` flag, Rust does *not* include checks for integer overflow that cause panics and
performs *two's complement wrapping*. For example in the case of a `u8` with the range `[0, 255]`, 
the value 256 becomes 0, the value 257 becomes 1, and so on.

Handle the possibility of overflow:
- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
- Return the `None` value if there is overflow with the `checked_*` methods.
- Return the value and a Boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value's minimum or maximum values with the `saturating_*` methods.

### Floating-point numbers

Rust has floating-point types `f32` and `f64`, which are represented according to the IEEE-754 standard and 
are 32 bits and 64 bits in size, respectively.
The default type is `f64`.

### The Boolean type

The Boolean type in Rust is `bool` and has two possible values: `true` and `false`.

### The character type

This is Rust's `char` type.
The `char` type is four bytes in size and represents a Unicode scalar value.
*Single quotes* must be used for the `char` literals, while *double quotes* must be used for string literals.

## Stack-allocated compound types

Tuples and arrays are stack-allocated when their elements stored on the stack.

### The tuple type

- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- The types of the different values in the tuple don't have to be the same.
- The tuple without any values has a special name, *unit*. 
This value and its corresponding type are both written `()` and represent an empty value or an empty return type. 
Expressions implicitly return the unit value if they don't return any other value.

### The array type

- An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
- Arrays in Rust have a fixed length.
- Every element of an array must have the same type.
- An array isn't as flexible as the *vector* type that is
provided by the standard library that is allowed to grow or shrink in size.

## Types implementing the `Copy` trait

- Rust has a special annotation called the `Copy` trait for types 
that are stored on the stack, as integers are.
- If a type implements the `Copy` trait, variables that use it do not move, but rather
are trivially copied, making them still valid after assignment to another variable.
- Rust won't let us annotate a type with `Copy` if the type, or any of its parts, 
has implemented the `Drop` trait.

Examples of the types that implement `Copy`:
- all the integer types
- the Boolean type, `bool`
- all the floating-point types
- the character type, `char`
- tuples, if they only contain types that also implement `Copy`, for example,
`(i32, i32)` implements `Copy`, but `(i32, String)` does not.

# Data types managing data allocated on the heap 

### The String type

The `String` type manages data allocated on the heap and 
is able to store an amount of text that is not defined at compile time.
But the `String` itself (the `String` struct: pointer + length + capacity) is stack-allocated.

```rust
let s1 = String::from("hello");
```

- A `String` is made up of three parts: 
    - a pointer to the memory that holds the contents of the string, 
    - a length = the memory of the contents in bytes, and 
    - a capacity = the total amount of memory in bytes received from the allocator.


- part of data stored on the stack:

| name     | value                                |
|----------|--------------------------------------|
| ptr      | <pointer to the index 0 on the heap> |
| len      | 5                                    |
| capacity | 5                                    |

- part of data stored on the heap:

| index | value |
|-------|-------|
| 0     | `h`   |
| 1     | `e`   |
| 2     | `l`   |
| 3     | `l`   |
| 4     | `o`   |


### *Moving* the variable

*Move* the variable that means,
copy the pointer, the length, and the capacity to the memory on the stack, 
and don't copy the data on the heap that the pointer refers to.

```rust
let s2 = s1;
// `s1` is no longer valid
```

`s1` was invalidated that means, it is no longer valid.
We say: `s1` was *moved* into `s2`.
Rust will never automatically create "deep" copies.

# Other types

### &'static str

```rust
let s = "hello";
```
The type of `s` is `&'static str`:
- `&str` is a string slice type (borrowed reference to some UTF-8 text);
- `'static` denotes the lifetime meaning *"this data lives for the entire duration of the program"* 
(because string literals are baked into the program's binary at compile time);
- `"hello"` is stored once in the binary at compile time (static memory / data section);
- `s` is a reference (`&str`), which lives on the stack (small pointer + length);
- But the actual string data `"hello"` is not copied to the stack or heap, it's pointing to that static memory.