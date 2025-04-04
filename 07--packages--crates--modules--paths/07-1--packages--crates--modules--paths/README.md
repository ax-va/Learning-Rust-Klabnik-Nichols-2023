# Packages, Crates, Modules, Paths

## Packages

A package is a collection of one or more crates.
A package can contain:
- zero or one *library crate*,
- any number of *binary crates*.

Create a package and see what Cargo creates
```
$ cd 07*
$ cd 07-01*
$ cargo new project
$ ls project
Cargo.toml  src
$ ls project/src
main.rs
```
Every package has a `Cargo.toml` file that describes how to build the crates it contains.

## Crates

A crate is the top-level compilation unit. 
When you compile a Rust program, you're compiling a crate.
It contains modules and submodules.

`src/main.rs` is the *crate root* of a *binary crate* with the same name as the package.
Similarly, `src/lib.rs` is the *crate root* of the *library crate*.
The crate root file is the root module.
Thus, the crate *starts at* the root module, and everything is built from there.
Cargo passes the crate root files to *rustc* to build the library or binary (executable).

A package can have multiple binary crates (with or without the main binary crate) 
by placing files in the `src/bin` directory: each file will be a separate binary crate.
Each binary crate is compiled into its own executable.

Binary crates can share the library crate's code through public APIs.

## Modules, submodules

Create a package
```
$ cargo new backyard
```

Then, create modules and submodules so that we have in the package
```
backyard
|-- scr
    |-- garden
        |-- vegetables.rs
    |-- garden.rs
    |-- main.rs
|-- Cargo.toml
```

- To make modules and items within the modules *public*, use the `pub` keyword.

    - In Rust, all items (functions, methods, structs, enums, modules, and constants) 
    are *private* to parent modules by default.
    - Items in a parent module cannot use the private items inside child modules,
    but items in child modules can use the items in their ancestor modules.

- To declare a module in a scope, use `mod <module_name>;` or `pub mod <module_name>;`.
- To use shortcut for a module's item in the scope of another module, use the `use` or `pub use` keyword, 
e.g., `use crate::garden::vegetables::Asparagus;` or `pub use crate::garden::vegetables::Asparagus;`.

## Paths

A *path* shows Rust where to find an item in a *module tree*.

- An *absolute path* is the full path starting from a crate root; 
for code from an external crate, the absolute path begins with the crate name, and
for code from the current crate, it starts with the literal `crate`.

- A *relative path* starts from the current module and uses `self`, `super`, or 
an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).
