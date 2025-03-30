/*
$ cd 07*
$ cd 07-01*
$ cargo new backyard
$ cd backyard
$ cargo run
 */

// shortcut to write `Asparagus` to make use of that type in the scope
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    // I'm growing Asparagus!
}
