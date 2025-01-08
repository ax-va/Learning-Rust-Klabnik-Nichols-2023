/*
```
$ cd 05*
$ cd 05-1*
$ cargo new project
$ cd project
```
 */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("ax-va"),
        email: String::from("ax-va@example.com"),
        sign_in_count: 1,
    };

    println!("{0}", user1.username);
    // ax-va
    println!("{0}", user1.email);
    // ax-va@example.com

    // The syntax `..` specifies that the remaining fields not explicitly set
    // should have the same value as the fields in the given instance.
    let user2 = User {
        username: String::from("alex-v"),
        ..user1  // Must come last
    };
    // We can no longer use `user1` after creating `user2` because
    // the `String` in the username field of `user1` was moved into `user2`.
    // If we had given `user2` new `String` values for both `email` and `username`,
    // and thus only used the `active` and `sign_in_count` values from `user1`,
    // then `user1` would still be valid after creating `user2`.
    // Both `active` and `sign_in_count` are types that implement the `Copy` trait.

    println!("{0}", user2.username);
    // alex-v
    println!("{0}", user2.email);
    // ax-va@example.com
}
