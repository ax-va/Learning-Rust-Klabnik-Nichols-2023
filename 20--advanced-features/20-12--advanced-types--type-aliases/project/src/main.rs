/*
The main use case for *type aliases* is to reduce repetition of lengthy types.
 */

// Example 1

fn main() {
    // Create the alias `Kilometers` to `i32`
    type Kilometers = i32;
    // Values that have the type `Kilometers`
    // will be treated the same as values of type `i32`.
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
    // x + y = 10
}

// Example 2

use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}