fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // mutable referensing
    let mut s = String::from("hello");
    change(s)
    // one caveat
    // we CANNOT borrow s as mutable more than once ( use brackets to allow multiple mutable references)

    {
        let r3 = &mut s;
    }

    // we CANNOT have a mutable and immutable reference to the same value (if scopes overlap)

    //DANGLING pointers ->
    let reference_to_nothing = dangle()
}
// creation of reference is BORROWING
fn calculate_length(s: &String) -> usize {
    s.len()
} // here s goes out of scope , but because it does not havbe ownership of what it refers to ,it is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world") // will n work only if we reference it as immutable value
}

fn dangle()->&String{
    let s = String::from("hello");
    &s // we return a reference to the String, s
}// Here, s goes out of scope, and is dropped. Its memory goes away.