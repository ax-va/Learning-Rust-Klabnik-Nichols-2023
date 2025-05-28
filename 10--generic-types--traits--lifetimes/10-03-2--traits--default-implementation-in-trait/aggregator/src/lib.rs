/*
*Traits* are similar to what many other languages call *interfaces*,
and they define shared functionality that types can implement.
Each type implementing a trait must provide its own implementation of trait's method signatures.
We can also define a default implementation in a trait.

We can implement a trait on a type in a crate *only* if either the trait or the type, or both, are local to the crate,
for example, the `Display` trait on the following `Tweet` type or the following `Summary` trait on the `Vec<T>` type.
But we cannot implement external traits on external types, for example, the `Display` trait `on Vec<T>`.
This restriction is part of a property called *coherence*, and more specifically the *orphan rule*.
This rule ensures that other people's code cannot break your code and vice versa.

```
$ cd 10*
$ cd 10-03-2*
$ cargo new aggregator --lib
$ cd aggregator
$ cargo build
```
 */

// trait to summarize information
pub trait Summary {
    // This method can be used in the default implementation
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String, // Assume that the tweet content is limited to 280 characters
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
