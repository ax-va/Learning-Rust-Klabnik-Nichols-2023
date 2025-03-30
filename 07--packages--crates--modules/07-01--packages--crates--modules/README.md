# Packages, Crates, Modules

## Crates

A *crate* can come in one of two forms: a *binary crate* or a *library crate*.

## Packages

A *package* *must* contain *at least* one crate, whether that's a library or binary crate, and
*can* contain *no or more* binary crates, but *at most* one library crate.

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

Cargo follows a convention that `src/main.rs` is the *crate root* of a *binary crate* with the same name as the package.
Similarly, `src/lib.rs` is the *crate root* of the *library crate*.
If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a binary and a library, 
both with the same name as the package.
Cargo passes the crate root files to *rustc* to build the library or binary.
A package can have multiple binary crates by placing files in the `src/bin` directory: 
each file will be a separate binary crate.

## (Sub)Modules

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

- To make modules and items within the modules public, use the `pub` keyword.
- To declare a module in a scope, use `pub mod <module_name>;`.
- To use shortcut for a module's item in the scope of another module, use `use crate::<relative_path_to>::<item>;`, 
e.g., `use crate::garden::vegetables::Asparagus;`.
