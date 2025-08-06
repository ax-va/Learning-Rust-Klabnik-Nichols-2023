# Smart Pointers

In Rust, smart pointers are data structures 
that not only act like pointers (i.e., they reference data in memory) 
but also have additional metadata and capabilities, 
such as automatic memory management, borrowing rules, and more.

Smart pointers in Rust differ from regular references (`&T` and `&mut T`) in that they:
- own the data they point to;
- implement the `Deref` and sometimes the `Drop` trait;
- often enable *heap allocation*, *shared ownership*, or *interior mutability*.

The `Deref` trait allows an instance of a smart pointer struct to behave like a reference, 
enabling code to work seamlessly with either references or smart pointers.
The `Drop` trait allows you to customize the behavior 
that runs when an instance of a smart pointer goes out of scope.

## Common Smart Pointers in Rust

1. `Box<T>`
  - heap-allocates data
  - single ownership
  - useful for recursive types or large data that you don't want on the stack.

    Use cases:
    - You have a type whose size can't be known at compile time, 
    and you want to use a value of that type in a context that requires an exact size.
    - You want to transfer ownership of a large value efficiently, 
    ensuring that only the pointer is moved and the data itself isn't duplicated.
    - You want to own a value, and you care only that it's a type 
    that implements a particular trait rather than being of a specific type.

2. `Rc<T>` (*Reference Counted*)
  - enables multiple ownership of data via reference counting;
  - used in single-threaded scenarios.

3. `Arc<T>` (*Atomic Reference Counted*)
  - like `Rc<T>`, but thread-safe via atomic operations;
  - used for sharing data across threads.

4. `RefCell<T>`
  - enables *interior mutability*: you can mutate the data even if the `RefCell<T>` is immutable;
  - enforces borrowing rules *at runtime* rather than compile time;
  - only for single-threaded scenarios.

5. `Mutex<T>` and `RwLock<T>`
  - used in concurrent programming for synchronized interior mutability;
  - `Mutex<T>` allows one writer;
  - `RwLock<T>` allows multiple readers or one writer.

| Smart Pointer | Ownership | Thread Safe | Mutability | Use Case                            |
|---------------|-----------|-------------|------------|-------------------------------------|
| `Box<T>`      | Single    | Yes         | Immutable  | Heap allocation                     |                     
| `Rc<T>`       | Shared    | No          | Immutable  | Shared data in single thread        |        
| `Arc<T>`      | Shared    | Yes         | Immutable  | Shared data in single thread        |      
| `RefCell<T>`  | Single    | No          | Mutable    | Runtime-checked interior mutability | 
| `Mutex<T>`    | Shared    | Yes         | Mutable    | Thread-safe mutable access          |  


