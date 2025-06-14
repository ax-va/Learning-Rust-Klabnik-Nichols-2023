/*
```
$ cd 06*
$ cd 06-5*
$ cargo new project
$ cd project
$ cargo run
```

Match patterns:
- `other` = the catch-all pattern,
- `_` = the catch-all pattern without using the value in the pattern

 */

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // the unit value, no code to run
    }

}
