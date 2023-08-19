use std::collections::HashMap;

#[derive(Debug)]
enum SpeadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // specifying type
    let v: Vec<i32> = Vec::new();

    // type is inferred by using macro
    let v1 = vec![1, 2, 3];
    // create empty vector
    let mut v2 = Vec::new();
    v2.push(2);
    v2.push(3);
    println!("{:?}", v2);

    let second: &i32 = &v2[1];
    println!("The second element is {second}");

    let second: Option<&i32> = v2.get(1);
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("There is no second element."),
    }

    for i in &v1 {
        println!("{i}")
    }

    let row = vec![
        SpeadsheetCell::Int(3),
        SpeadsheetCell::Text(String::from("blue")),
        SpeadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    hash_map()
}
// Strings!!

fn strings_section() {
    let mut s = String::new();

    let data = "initial contents";

    // they do the same thing!!!
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // add to string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    // concatentation
    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4; // note s3 has been moved here and can no longer be used (add function takes ownership of s3)
                       // println!("{:?}", s3); // this will not work

    // format! macro
    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");
    let s9 = format!("{}-{}-{}", s6, s7, s8); // this does not take ownership of any of the variables and works like println!

    // iterating over strings
    for char in "hello".chars() {
        println!("{}", char);
    }

    // iterating over bytes
    for byte in "hello".bytes() {
        println!("{}", byte);
    }
}

fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // retrieve value from hash map using copy trait and unwrap it to check if it exists
    let team_score = scores.get("Blue").copied().unwrap_or(0);
    print!("{:?}", team_score);

    // iterate over hash map
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // create hash map from vectors
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // updating hash maps
    // 1 - overwriting a value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // this will overwrite the previous value
    println!("{:?}", scores);

    // 2 - only inserting a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // this will not overwrite the previous value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // 3. updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns a mutable reference (&mut V) to the value for this key
        // if the key exists, and if not, inserts the parameter as the new value for
        // this key and returns a mutable reference to the new value.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
