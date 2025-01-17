/*
```
$ cd 06*
$ cd 06-3*
$ cargo new project
$ cd project
```
 */

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // match arms: <pattern> => <code>
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
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value: {}", value_in_cents(coin));
    // Lucky penny!
    // Value: 1
}
