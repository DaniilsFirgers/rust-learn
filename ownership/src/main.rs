fn main() {
    let mut s = String::from("hello"); // :: alows us to namespace from func under the String type
    s.push_str(", world!");
    println!("{}", s);
    // heap clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = String::from("hello");
    takes_ownership(s3);
    let x = 5;
    makes_copy(x);

    let s4 = gives_ownership();
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s2);
} // s3 goes out of scope and is dropped. s2 weas moved, so nothing happends to it. s1 goes out of scope and is dropped

fn takes_ownership(some_string: String) {
    // some string comes into scope
    println!("{}", some_string)
} // here some_string goes out of scope and 'drop' is called. The memory is freed

fn makes_copy(some_integer: i32) {
    // some integer comes into scope
    println!("{}", some_integer)
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
