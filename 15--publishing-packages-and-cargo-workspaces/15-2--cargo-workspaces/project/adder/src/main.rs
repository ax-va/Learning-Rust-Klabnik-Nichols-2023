use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("{num} plus one is {}.", add_one::add_one(num));
    // 10 plus one is 11.
    println!("{num} plus two is {}.", add_two::add_two(num));
    // 10 plus two is 12.
}