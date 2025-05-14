# Terms

### Associated Functions

All functions defined within an `impl` block are called `associated functions`.

### Borrowing

Creating a reference is called *borrowing*.

### Contracts

Functions often have *contracts*: their behavior is only guaranteed if the inputs meet particular requirements. 
Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug, 
and it is not a kind of error you want the calling code to have to explicitly handle.

### Dangling Pointer = a pointer that references a location in memory that may have been given to someone else

The memory can be freed up while preserving a pointer to that memory.
In Rust, the compiler guarantees that references will never be dangling references.

### Monomorphization

The compiler looks at all the places where generic code is called 
and generates code for the concrete types the generic code is called with.
When the code runs, it performs just as it would if we had duplicated each definition by hand.

### Ownership

*Ownership* is a set of rules that govern how a Rust program manages memory in the *heap*.
If any of the rules are violated, the program won't compile.

### Prelude

By default, Rust has a set of items defined in the standard library.
This set, called the *prelude*, will be brought into the scope of every program.

See also:

https://doc.rust-lang.org/std/prelude/index.html

### RAII = Resource Acquisition Is Initialization

In C++, the pattern of deallocating resources at the end of an item's lifetime 
is sometimes called *Resource Acquisition Is Initialization (RAII)*. 
The `drop` function in Rust will be familiar to you if you've used RAII patterns.
Rust calls the `drop` function automatically at the closing curly bracket `}` to return the memory to the allocator.

### Traits

*Traits* are similar to what many other languages call *interfaces*, 
and they define shared functionality that types can implement.
Each type implementing a trait must provide its own implementation of trait's method signatures.
