**borrowing**

Creating a reference is called *borrowing*.

**ownership**

*Ownership* is a set of rules that govern how a Rust program manages memory in the *heap*.
If any of the rules are violated, the program won't compile.

**prelude**

By default, Rust has a set of items defined in the standard library.
This set, called the *prelude*, will be brought into the scope of every program.

See also:
https://doc.rust-lang.org/std/prelude/index.html

**RAII = Resource Acquisition Is Initialization**

In C++, the pattern of deallocating resources at the end of an item's lifetime 
is sometimes called *Resource Acquisition Is Initialization (RAII)*. 
The `drop` function in Rust will be familiar to you if you've used RAII patterns.
Rust calls the `drop` function automatically at the closing curly bracket `}` to return the memory to the allocator.