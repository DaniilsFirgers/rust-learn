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

    println!("{:?}", row)
}
