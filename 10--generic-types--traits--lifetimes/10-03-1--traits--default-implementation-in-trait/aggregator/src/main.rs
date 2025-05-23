/*
```
$ cd 10*
$ cd 10-03-1*
$ cd aggregator
$ touch scr/file.txt
$ cargo run
```
 */

// The `Summary` trait must also be in scope
use aggregator::{Summary, NewsArticle};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL."
        ),
    };

    println!("New article available! {}", article.summarize());
    // New article available! (Read more...)
}