enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

// methods on enum using impl

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
// TADAM
