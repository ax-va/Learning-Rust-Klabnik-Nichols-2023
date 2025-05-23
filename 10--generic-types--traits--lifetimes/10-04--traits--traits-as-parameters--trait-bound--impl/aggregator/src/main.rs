/*
```
$ cd 10*
$ cd 10-04*
$ cd aggregator
$ touch scr/file.txt
$ cargo run
```
 */

// The `Summary` trait must also be in scope
use aggregator::{Summary, Tweet, NewsArticle, notify_v1, notify_v2};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people ..."),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL."
        ),
    };

    notify_v1(&tweet);
    // Breaking news! horse_ebooks: of course, as you probably already know, people ...

    notify_v1(&article);
    // Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)

    notify_v2(&tweet);
    // Breaking news! horse_ebooks: of course, as you probably already know, people ...

    notify_v2(&article);
    // Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)
}