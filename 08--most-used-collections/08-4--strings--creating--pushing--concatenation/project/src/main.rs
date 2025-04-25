fn main() {
    // Create a mutable, empty string instance
    let mut s = String::new();

    // Create a string from a string literal:

    // 1. by using the `to_string` method from the `Display` trait.
    let data = "initial contents";
    let s = data.to_string();
    // or
    let s = "initial contents".to_string();

    // 2. by using the `String::from` method
    let s = String::from("initial contents");

    // UTF-8 encoding
    let hello = String::from("Hallo!");
    let hello = String::from("こんにちは!");
    let hello = String::from("안녕하세요!");
    let hello = String::from("你好!");
    let hello = String::from("Здравствуйте!");
    let hello = String::from("Hola!");

    // Append a string literal
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");
    // s is foobar

    // Explanation:
    // - "bar" is borrowed as a string slice argument to `push_str`;
    // - once the method finishes, the borrow ends;
    // - `push_str` copies, not moves or borrows long-term;
    // - the `String` and `&'static str` instances own their data.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    // s1 is foobar
    println!("s2 is {s2}");
    // s2 is bar

    // Append a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");
    // s is lol

}
