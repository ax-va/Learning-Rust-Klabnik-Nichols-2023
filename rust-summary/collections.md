# Collections

Rust's standard library includes a number of very useful data structures called *collections*.
Unlike the built-in *array* and *tuple* types, the collections' data is stored on the *heap*, 
which means the amount of data does not need to be known at compile time.

https://doc.rust-lang.org/std/collections/index.html

## Most used collections

### Vectors

Vectors are implemented using generics and can only store values of the same type.
However, when we need one type to represent elements of different types, we can define and use an enum.

The `Vec<T>` type is a growable, heap-allocated vector 
that stores elements of type `T` in a contiguous block of memory.
It provides dynamic resizing, allowing elements to be added or removed efficiently, 
and supports indexing, iteration, and a variety of useful methods for manipulation.

Like any other `struct`, a vector is freed when it goes out of scope
```rust
{
    let vector = vec![1, 2, 3, 4];
    // do stuff with `vector`
} // `vector` goes out of scope; the integers it holds will be cleaned up
```

Vectors are one of the most commonly used collection types in Rust due to their flexibility and performance.

https://doc.rust-lang.org/nomicon/vec/vec.html

### Strings

The `String` type, which is provided by Rust's standard library rather than coded into the core language, 
is a growable, mutable, owned, UTF-8 encoded string type.
It provides methods for appending, slicing, and modifying text, 
and can be easily converted from the string slice `&str` type (that is also UTF-8 encoded).

- A `String` owns its heap-allocated data, 
whereas a `&str` is a borrowed reference to some string data (without ownership).

- `String` is actually implemented as a wrapper around a vector of bytes 
with some extra guarantees, restrictions, and capabilities.

`String` is ideal for situations where you need to own and mutate string data at runtime.

### Hash maps

`HashMap<K, V>` is a hash table-based collection that stores key-value pairs, 
allowing for efficient data retrieval based on keys. 
It provides fast insertion, lookup, and removal of elements, with keys being unique within the map. 
`HashMap` is commonly used when you need to associate data with unique identifiers 
and supports flexible hashing strategies through the Hash and Eq traits.
