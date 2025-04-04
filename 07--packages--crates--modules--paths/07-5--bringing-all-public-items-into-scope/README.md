# Packages, Crates, Modules, Paths

## The `*` Glob Operator

Bring all public items defined in a path into scope with the `*` glob operator

```rust
use std::collections::*;
```

The glob operator is often used when testing to bring everything under test into the `tests` module.
