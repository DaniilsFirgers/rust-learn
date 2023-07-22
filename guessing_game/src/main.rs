// import not preluded library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess!");

        let mut guess = String::new(); // create a new mutable guess string called by new associated func

        io::stdin()
            .read_line(&mut guess) // read user's input and append it to guess reference (&)
            .expect("Failed to read the line"); // call expect method that will return arhument passed as error if Err is indicated

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // first arm
            Err(_) => continue, // second arm
        }; // use shadowing to create a variabel of differnet type with the same name
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // match secretNumber
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break from loop
            }
        }
    }
}
