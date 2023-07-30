fn main() {
    let s = String::from("hello world");

    let len = s.len();
    let _slice = &s[3..len]; // intentionally unused
    let _slice1 = &s[..]; // or &s[0..len]

    let first_word_str = first_word(&s);
    println!("first word {first_word_str}");

    // array slices
    let array1 = [1, 2, 3, 4, 5];
    let _slice = &array1[1..3];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
