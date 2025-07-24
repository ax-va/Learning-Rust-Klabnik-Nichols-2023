# Public Crates

https://crates.io

## Example: rand

https://crates.io/crates/rand

## Setting Up a Crates.io Account

- Create an account on https://crates.io via a GitHub account (`Log in with GitHub`).

- Visit your account settings at https://crates.io/me and retrieve an API *secret* token.

- Run in the terminal
```unix
$ cargo login <api_token>
       Login token for `crates-io` saved
```
This command will inform Cargo of your API token and store it locally in `~/.cargo/credentials`.

## Adding Metadata to a New Crate

Package names must be unique across Crates.io.
Set a package name in `Cargo.toml`. 
To publish your crate, you must include at least a description and a license.
For the license field, you need to give a license identifier value.
The Linux Foundation's Software Package Data Exchange (SPDX) at https://spdx.org/licenses 
lists the identifiers you can use for this value.
```unix
[package]
name = "<package_name>"
...
description = "<descriptio>"
license = "MIT OR Apache-2.0"
...
```
