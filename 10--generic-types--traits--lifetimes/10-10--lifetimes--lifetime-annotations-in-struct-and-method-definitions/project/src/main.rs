/*
```
$ cd 10*
$ cd 10-10*
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

// Lifetime names for struct fields always need to be
// 1. declared after the `impl` keyword
// 2. and then used after the struct's name.
impl<'a> ImportantExcerpt<'a> {
    // The first elision rule applies to each parameter that is a reference
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // The third lifetime elision rule applies to the output lifetime parameter
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

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
