fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
    test();
    let ten = implicit_return();
    let x = plus_one(5);
    println!("{ten}");
    println!("{x}")
}

fn another_function(x: i32, unit_label: char) {
    println!("the value of x is {x} and char is {unit_label}")
}

fn test() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");
}

fn implicit_return() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // ; here iwll produce error
}

// if we put ; at the end of the line then it will not return anytthing and error will pop up
