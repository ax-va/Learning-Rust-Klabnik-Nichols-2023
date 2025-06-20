/*
```
$ cd 10*
$ cd 10-04*
$ cargo new aggregator --lib
$ cd aggregator
$ cargo build
```

How to use traits to define functions that accept many different types.

 */

// trait to summarize information
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String, // Assume that the tweet content is limited to 280 characters
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// This is actually *syntax sugar*
pub fn notify_v1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// for a longer form known as a *trait bound*
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*`where` clauses for trait bounds */

// Instead of
/*
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
}
```
 */
// use a `where` clause for clearer trait bounds
/*
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```
 */
