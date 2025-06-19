/*
```
$ cd 13*
$ cd 13-1*
$ cargo new project
$ cd project
$ cargo run
```
 */

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(
        &self,
        user_preference: Option<ShirtColor>, // The `Option` has two variants: `Some` or `None`
    ) -> ShirtColor {
        // In the case of the`None` variant,
        // `unwrap_or_else` calls the closure
        // that takes no parameters itself.
        // The closure calls `self.most_stocked()`.
        user_preference.unwrap_or_else(|| self.most_stocked())
        // Closures in Rust can capture values from their environment.
        // In this case, the closure `|| self.most_stocked()` captures `self` by borrowing it,
        // so it can use it inside the closure body.
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ],
    };

    let user_pref_1 = Some(ShirtColor::Red);
    let giveaway_1 = store.giveaway(user_pref_1);
    println!("The user with preference `{:?}` gets `{:?}`.", user_pref_1, giveaway_1);
    // The user with preference `Some(Red)` gets `Red`.

    let user_pref_2 = None;
    let giveaway_2 = store.giveaway(user_pref_2);
    println!("The user with preference `{:?}` gets `{:?}`.", user_pref_2, giveaway_2);
    // The user with preference `None` gets `Blue`.
}
