/*
```
$ cd 10*
$ cd 10-4*
$ cd aggregator
$ touch scr/file.txt
$ cargo run
```
 */

// The `Summary` trait and the `Tweet` struct *must not* be in scope
use aggregator::{notify, return_summarizable};

fn main() {
    let tweet = return_summarizable();
    notify(&tweet);
    // Breaking news! horse_ebooks: of course, as you probably already know, people ...
}