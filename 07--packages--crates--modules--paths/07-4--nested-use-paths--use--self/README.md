# Packages, Crates, Modules, Paths

## Nested `use` Paths

Use a nested path at any level in a path, which is useful when combining `use` statements that share a subpath.

- Example 1

    ```rust
    use std::{cmp::Ordering, io};
    ```
    is equivalent to
    ```rust
    use std::cmp::Ordering;
    use std::io; // equivalent to `use std::io::self;`
    ```

- Example 2

    ```rust
    use std::io::{self, Write};
    ```
    is equivalent to
    ```rust
    use std::io;
    use std::io::Write;
    ```
- You *cannot* continue the path after the closing `}` in that syntax

    ```rust
    use std::io::{self, Write}::SomethingElse;
    ```
