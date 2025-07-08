# More About Cargo

See also:

- https://doc.rust-lang.org/cargo

## Customizing Builds with Release Profiles

In Rust, *release profiles* are predefined and customizable sets of configurations 
that give programmers greater control over compilation options. 
Each profile is configured independently of the others.
Rust provides two main release profiles: 

- `dev`, optimized for faster compilation during development;
- `release`, optimized for performance in production builds.

```unix
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
...
```

```unix
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
...
```

We can override default settings of release profiles in `Cargo.toml` and customize them
```unix
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
- for development: `opt-level = 0` (default for `dev`) means *no optimizations* are applied, 
which makes compilation faster but the code runs slower;

- for production builds: `opt-level = 3` (default for `release`) enables *maximum optimizations*, 
making the binary faster at runtime but increasing compile time.

See also:

- https://doc.rust-lang.org/cargo/reference/profiles.html
