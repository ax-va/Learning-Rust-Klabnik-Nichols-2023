/*
Using `@` lets us test a value and save it in a variable within one pattern.
 */

enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // `3..=7` means an inclusive range [3, 7].
            // `id_var @ 3..=7` means "Match values in this range and bind the matched value to `id_var`".
            id: id_var @ 3..=7,
        } => println!("Found an id in range: {id_var}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Some other id: {id}"),
    }
    // Found an id in range: 5
}
