/*
```
$ cd 10*
$ cd 10-1*
$ cargo new project
$ cd project
$ cargo run
```
 */

// generics in function definitions

// Explanation for `get_max`:
// - The function is generic over some type `T`;
// - The function has one parameter, which is a slice of values of type `T`;
// - The function will return a reference to a value of the same type `T`;
// - The types valid for `T` are restricted to be compared in the function.
fn get_max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }

    max
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65, 11];
    let result = get_max(&number_list);
    println!("The maximum number is {result}");
    // The maximum number is 100

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_max(&char_list);
    println!("The maximum char is {result}");
    // The maximum char is y
}
