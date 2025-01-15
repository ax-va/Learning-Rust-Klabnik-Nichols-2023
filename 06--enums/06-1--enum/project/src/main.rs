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

fn route(ip_kind: IpAddrKind) {}

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

}
