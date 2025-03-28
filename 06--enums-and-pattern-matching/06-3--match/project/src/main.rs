/*
```
$ cd 06*
$ cd 06-3*
$ cargo new project
$ cd project
$ cargo run
```
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

fn value_in_cents_v1(coin: Coin) -> u8 {
    match coin {
        // match arms:
        // `<pattern> => <code>,`
        // If a pattern matches the value,
        // the code associated with that pattern is executed.
        // If that pattern doesn't match the value,
        // execution continues to the next arm.
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

fn value_in_cents_v2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // The `state` variable will bind to the value of that quarter's state
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value: {}", value_in_cents_v1(coin));
    // Lucky penny!
    // Value: 1

    value_in_cents_v2(Coin::Quarter(UsState::Alaska));
    // State quarter from Alaska!
}
