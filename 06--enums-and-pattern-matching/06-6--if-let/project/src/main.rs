/*
```
$ cd 06*
$ cd 06-6*
$ cargo new project
$ cd project
$ cargo run
```

The `if let` syntax lets us handle values that match one pattern while ignoring the rest.
The code in the `if let` block isn't run if the value doesn't match the pattern.
There is also `if let ... else`.

 */

 #[derive(Debug)] // to inspect the state
 enum UsState {
    Alabama,
    Alaska,
    // other US States
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // associated enum
}

fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // The maximum is configured to be 3

    // the same behaviour as `match`, the value `max` can be used inside the body
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // The maximum is configured to be 3

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // State quarter from Alaska!

    // Notice:
    // `state` is a value of enum that implements the `Debug` trait.
    // `{:?}` tells `println!` to print `state` in a human-readable debug format.

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    // the same behaviour as `match`
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // State quarter from Alaska!
}
