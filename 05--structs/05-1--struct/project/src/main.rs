/*
```
$ cd 05*
$ cd 05-1*
$ cargo new project
$ cd project
$ cargo run
```
 */

struct User {
    // Define fields
    // `<field_name>: <type>,`
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email, // field init shorthand
        sign_in_count: 1,
    }
}

fn main() {
    // immutable struct
    let user1 = User {
        active: true,
        username: String::from("ax-va"),
        email: String::from("ax-va@example.com"),
        sign_in_count: 1,
    };

    println!("{0}", user1.username);
    // ax-va

    // mutable struct
    let mut user2 = User {
        active: true,
        username: String::from("ax-va"),
        email: String::from("ax-va@example.com"),
        sign_in_count: 1,
    };

    // Notation:
    // The entire instance must be mutable.
    // Marking only certain fields as mutable is not allowed.

    user2.username = String::from("alex-v");
    println!("{0}", user2.username);
    // alex-v

    let user3 = build_user(String::from("axx-va"), String::from("ax-va@example.com"));
    println!("{0}", user3.username);
    // axx-va
}
