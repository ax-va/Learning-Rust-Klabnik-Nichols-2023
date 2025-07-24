# Public Crates

https://crates.io

## Example: rand

https://crates.io/crates/rand

## Publishing packages

### Setting Up a Crates.io Account

- Create an account on https://crates.io via a GitHub account (`Log in with GitHub`).

- Visit your account settings at https://crates.io/me and retrieve an API *secret* token.

- Run in the terminal
```unix
$ cargo login <api_token>
       Login token for `crates-io` saved
```
This command will inform Cargo of your API token and store it locally in `~/.cargo/credentials`.

- A verified email address is required to publish crates to Crates.io.  
Visit https://crates.io/settings/profile to set and verify your email address.

### Adding Metadata to a New Crate

Package names must be unique across Crates.io.
Set a package name in `Cargo.toml`. 
To publish your crate, you must include at least a description and a license.
For the license field, you need to give a license identifier value.
The Linux Foundation's Software Package Data Exchange (SPDX) at https://spdx.org/licenses 
lists the identifiers you can use for this value.
```unix
[package]
name = "<package_name>"
version = "<version>"
...
description = "<descriptio>"
license = "MIT OR Apache-2.0"
...
```
Use the *Semantic Versioning* rules at https://semver.org for versioning.

### Publishing to Crates.io

Notice: publishing a package version is *permanent* that means
*the version cannot be overwritten or be deleted*.

The changes must be commited to publish
```unix
$ cargo publish
```

### Yanking

A *yank* means that all projects with a `Cargo.lock` will not break, 
and any future `Cargo.lock` files generated will not use the yanked version.
A yank *does not* delete any code.

Yank
```unix
$ cargo yank --vers 1.0.1
```

Undo a yank
```unix
$ cargo yank --vers 1.0.1 --undo
```

