# Packages, Crates, Modules

## Using Third-Party Packages

Add this in `Cargo.toml` under `[dependencies]` to download the `rand` package 
and any dependencies from https://crates.io, and make `rand` available to the project.
```toml
# The specifier "0.8.5" is actually shorthand for "^0.8.5",
# which means any version that is at least 0.8.5 but below 0.9.0.
rand = "0.8.5"
```
When your Rust project depends on multiple versions of the same crate, 
Cargo will download and store each specified version separately on your machine. 
This ensures that each part of your project or its dependencies uses the version it requires.
If different parts of the dependency graph require incompatible versions of the same crate 
(e.g., one requires version 0.7 and another requires version 0.8), Cargo will include both versions in the build. 
This approach maintains compatibility across your project's dependencies.

For instance, if one dependency requires rand version 1.1 and another requires 1.2, 
Cargo can resolve this to use version 1.2 for both, since these versions are semver-compatible. 
But if one dependency requires version 0.7 and another requires 0.8, 
Cargo will include both versions because they are considered semver-incompatible.

To manage multiple versions of a crate explicitly, you can rename dependencies in `Cargo.toml` under `[dependencies]`. 
This allows you to specify different versions and refer to them with unique names in your code.
```toml
rand_v07 = { package = "rand", version = "0.7" }
rand_v08 = { package = "rand", version = "0.8" }
```

Use them in the code
```rust
use rand_v07::Rng as Rng07;
use rand_v08::Rng as Rng08;
```

## Comparison to Python: Dependency Hell

In Python, managing projects that require two packages with conflicting dependencies 
presents significant challenges due to the language's handling of package versions. 
Unlike some other ecosystems, Python does not natively support 
loading multiple versions of the same package within a single environment. 
This limitation means that when two packages depend on different, incompatible versions of the same dependency, 
conflicts arise that are not straightforward to resolve.
