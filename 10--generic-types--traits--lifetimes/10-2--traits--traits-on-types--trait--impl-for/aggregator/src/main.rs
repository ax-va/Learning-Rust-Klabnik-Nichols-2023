/*
```
$ cd 10*
$ cd 10-2*
$ cd aggregator
$ touch scr/file.txt
$ cargo run
```
 */

// The `Summary` trait must also be also in scope
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people ..."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // 1 new tweet: horse_ebooks: of course, as you probably already know, people ...
}