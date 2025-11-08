use gui::Draw;

// Someone can use our library to add a new GUI component

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!(
            "A select box is drawn with width = {:?}, height = {:?}, options = {:?}",
            self.width, self.height, self.options
        )
    }
}

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            // select box
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            // button
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // If a type does not implement the `Draw` trait
    // and we try to use it where `Draw` is required,
    // the compiler produces an error.

    // compilation error: "error[E0277]: the trait bound `String: Draw` is not satisfied"
    /*
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
        //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
    };

    screen.run();
    /*
}
// A select box is drawn with width = 75, height = 10, options = ["Yes", "Maybe", "No"]
// A button is drawn with width = 50, height = 10, label = "OK"
