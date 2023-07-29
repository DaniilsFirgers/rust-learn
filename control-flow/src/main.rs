fn main() {
    // if condition
    let number = 3;
    let condition = true;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisble by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2")
    }
    let number2 = if condition { 5 } else { 6 };

    // while loop

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number3 = 3;

    while number3 != 0 {
        println!("number - {number3}");
        number3 -= 1;
    }

    println!("LIFTOFF!");

    // loop thropugh a collection

    let array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is : {}", array[index]);

        index += 1;
    }

    for element in array {
        println!("the value is {element}")
    }
    for number in (1..4).rev() {
        println!("{number}!")
    }
}
