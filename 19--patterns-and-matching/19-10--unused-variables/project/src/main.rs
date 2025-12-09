fn main() {
    let _x = 5; // no warning about not using `_x`
    let y = 10; // "warning: unused variable: `y`"

    let s = Some(String::from("Hello!"));
    // The value from `s` will be moved into `_s`
    if let Some(_s) = s {
        println!("found a string");
    }
    // found a string

    // compilation error: "error[E0382]: borrow of partially moved value: `s`"
    /*
    println!("{:?}", s);
                     ^ value borrowed here after partial move
     */

    let s = Some(String::from("Hello!"));
    // The value from `s` will not be moved
    if let Some(_) = s {
        println!("found a string");
    }
    // found a string

    println!("{:?}", s);
    // Some("Hello!")
}
