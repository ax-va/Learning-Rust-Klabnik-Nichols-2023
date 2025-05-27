/*
```
$ cd 10*
$ cd 10-09*
$ cargo new project
$ cd project
$ cargo run
```
 */

struct ImportantExcerpt<'a> {
    // The struct has the single field part
    // that holds a string slice, which is a reference.
    part: &'a str,
}
// An instance of `ImportantExcerpt` cannot outlive the reference
// it holds in its part field.

fn main() {
    // owner
    let novel = String::from(
        "Call me Ishmael.\nSome years ago..."
    );

    // string slice, i.e. reference
    let first_line = novel
        .split('\n')
        .next()
        // If `next()` returns `None` (which won't happen here),
        // it will panic with the given message.
        .expect("Could not find a '\\n'");

    let i = ImportantExcerpt {
        part: first_line,
    };
}
