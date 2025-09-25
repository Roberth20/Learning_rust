// The enumeratios or enum are used to represent a value have different possible types
// For example, ip address only can be V4 or v6
enum IpAddrKind {
    V4,
    V6
}
// Also, we can define the data types that will be stored
enum IpAddrKind2 {
    V4(String),
    V6(String)
}
// Using enums also allow to have diffetent types of data associated
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

// Another example with more variety
enum Message {
    Quit, // No data associated
    Move{x: i32, y: i32}, // Named fields like structures
    Write(String), // A single string
    ChangeColor(i32, i32, i32) // Three numeric values
}
// This works like individual structs but grouped by a simple type
// Just like structs, we can define methods in enums
impl Message {
    fn call(&self) {
    }
}

fn main() {
    // Then, we can use the custom type for each case
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // So, we can call the function with both types
    route(four);
    route(six);

    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 0);
    let loopback = IpAddr::V6(String::from("::1"));

    let message = Message::Write(String::from("hello"));
    message.call();
}

// As the enums represent both types, it's easier to use in fucntions
fn route(ip_kind: IpAddrKind) {}