# Terms

### Associated Functions

All functions defined within an `impl` block are called `associated functions`.

### Borrowing

*Borrowing* in Rust allows one part of code to access data without taking ownership of it, using references. 
There are two types of borrowing: *immutable (read-only)* and *mutable (read-write)*, 
but you can’t have both at the same time for the same data. 
This system ensures memory safety and prevents data races by enforcing these rules at compile time.

### Borrow Checker

The *borrow checker* is a component of the Rust compiler that enforces the rules of ownership, borrowing, 
and lifetimes to ensure memory safety without needing a *garbage collector*. 
It ensures that references do not outlive the data they point to 
and that you can't have mutable and immutable references to the same data at the same time. 
This prevents common bugs like *data races*, *dangling pointers*, and *null references* at compile time.

### Contracts

Functions often have *contracts*: their behavior is only guaranteed if the inputs meet particular requirements. 
Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug, 
and it is not a kind of error you want the calling code to have to explicitly handle.

### Dangling References / Dangling Pointers

The memory can be freed up while preserving a pointer to that memory.
In Rust, *dangling references* occur when a reference points to memory that has been freed or is no longer valid
This can lead to undefined behavior in other languages. 
Rust prevents this at compile time using its ownership and borrowing rules, 
ensuring that references cannot outlive the data they point to.

In particular, returning references to local function variables results in a compile-time error in Rust.
This is because local variables are dropped when the function returns, 
and Rust's *borrow checker* ensures you can't return references to that invalid memory.

Examples:

- not allowed in Rust
  ```rust
  fn some_function() -> String {
      let some_string = String::from("some string");
      some_string.as_str() // Tries to return a reference to `some_string`
  }
  ```

- allowed in Rust
  ```rust
  fn some_function() -> String {
      let some_string = String::from("some string");
      some_string // Returns the owned `String` by value, transferring ownership of `some_string` to the caller.
  } // `some_string` is moved out of the function (not referenced), there's no risk of a dangling reference.
  ```

### grep = (g)lobally search a (r)egular (e)xpression and (p)rint

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

### Test-Driven Development (TDD)

TDD Cycle (Red → Green → Refactor):

1. **Write a Failing Test (Red)**
   - Purpose: Define the expected behavior of a small piece of functionality.
   - Why it should fail: To ensure the test is valid and that the feature doesn't already exist.
   - Mindset: You're designing the interface or behavior before implementation.
 

2. **Make the Test Pass (Green)**
   - Goal: Implement just enough code to make the test pass.
   - Don't overengineer: Avoid adding anything more than what is required.
 
 
3. **Refactor (Clean Up)**
   - Purpose: Improve structure, readability, or performance without changing behavior.
   - Key: All tests must still pass after refactoring.


### Semantic Versioning

Use the *Semantic Versioning* rules at https://semver.org for versioning.

### Traits

*Traits* are similar to what many other languages call *interfaces*, 
and they define shared functionality that types can implement.
Each type implementing a trait must provide its own implementation of trait's method signatures.

### Zero-Cost Abstractions

Example: 
Iterators are one of Rust's *zero-cost abstractions*, 
by which we mean that using the abstraction imposes no additional runtime overhead.
