# Data types

Rust is a *statically typed* language, which means that it must know the types of all variables at compile time.

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

## Floating-point numbers

Rust has floating-point types `f32` and `f64`, which are represented according to the IEEE-754 standard and 
are 32 bits and 64 bits in size, respectively.
The default type is `f64`.

## The Boolean type

The Boolean type in Rust is `bool` and has two possible values: `true` and `false`.

## The character type

This is Rust's `char` type.
