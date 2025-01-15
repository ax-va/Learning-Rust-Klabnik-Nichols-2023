/*
```
$ cd 06*
$ cd 06-1*
$ cargo new project
$ cd project
```
 */

// enumeration type with two possible values
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

// Example:
// enum in struct (with two fields) as data type.
// The example is less concise as below.
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

// Example:
// The definition of the `IpAddr` enum says that both
// `V4` and `V6` variants will have associated `String` values.
// The example is more concise as above.
enum IpAddr2 {
    V4(String),
    V6(String),
}

// Each variant can have different types and amounts of associated data
enum IpAddr3 {
    // Four numeric components that will have values between 0 and 255
    V4(u8, u8, u8, u8),
    V6(String),
}

// Example: structs as parameter types

struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddr5 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Another example: different annotation
enum Message {
    Quit, // no data associated
    Move { x: i32, y: i32 }, // named fields like in `struct`
    Write(String), // single `String`
    ChangeColor(i32, i32, i32), // three `i32` values
}

// associated functions by using `impl`
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    // Both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type `IpAddrKind`
    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // less concise

    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // more concise

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
