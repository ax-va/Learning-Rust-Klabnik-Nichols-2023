fn print_coordinates_v1((x, y): (i32, i32)) {
    println!("x = {x}, y = {y}");
}

fn print_coordinates_v2(&(x, y): &(i32, i32)) {
    println!("x = {x}, y = {y}");
}

fn main() {
    let point = (3, 5);

    print_coordinates_v1(point);
    // x = 3, y = 5

    let (x, y) = point;
    println!("x = {x}, y = {y}");
    // x = 3, y = 5

    print_coordinates_v2(&point);
    // x = 3, y = 5
}