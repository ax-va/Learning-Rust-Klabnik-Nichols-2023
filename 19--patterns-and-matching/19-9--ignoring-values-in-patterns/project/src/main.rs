/*
There are a few ways to ignore entire values or parts of values in a pattern:
- using the `_` pattern,
- using the `_` pattern within another pattern,
- using a name that starts with an underscore, or
- using `..` to ignore remaining parts of a value.
 */

// --------------------------------- //
// Ignoring an entire value with `_` //
// --------------------------------- //

// Completely ignore a value passed as the first argument
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
    // This code only uses the y parameter: 4

    // ------------------------------------------- //
    // Ignoring parts of a value with a nested `_` //
    // ------------------------------------------- //

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // Cannot overwrite the setting value until it is unset
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting_value = {:?}", setting_value);
    // setting_value = Some(5)

    // Unset the setting value
    setting_value = None;
    // Can overwrite the setting value now
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting_value = {:?}", setting_value);
    // setting_value = Some(10)

}
