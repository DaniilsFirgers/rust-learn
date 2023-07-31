enum IpAddr {
    V4(String)
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32,i32),
}

fn main() {
   let four = IpAddr::V4;
   let six IpAddr::V6;

   let home = IpAddr::V4(String::from("127.0.0.1"));
   let loopback = IpAddr::V6(String:from("::1"))
}

// methods on enum using impl

impl Message {
    fn call(&Message){
        // method body would be defined here
    }
}
// TADAM
let m = Message::Write(String::from("hello"));
m.call()
