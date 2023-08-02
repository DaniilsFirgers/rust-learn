mod structs;

use colored::*;
use std::env;
use std::io;
use std::process;
use structs::CommandLineOutput;

fn main() {
    let arguments = read_command_line();
    println!("{:?}", arguments)
}

fn read_command_line() -> CommandLineOutput {
    let mut input_amout = String::new();
    let mut input_home_currency = String::new();
    let mut input_foreign_currency = String::new();

    println!("Enter the exchange amount: ");
    match io::stdin().read_line(&mut input_amout) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to read input: {}", err);
        }
    }

    let amount = match input_amout.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Could not parse amount into numberic value");
            0.0
        }
    };

    println!("Enter the home currency: ");
    match io::stdin().read_line(&mut input_home_currency) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to read home currency input: {}", err);
        }
    }

    println!("Enter the foreign currency: ");
    match io::stdin().read_line(&mut input_foreign_currency) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to read home foreign input: {}", err);
        }
    }

    CommandLineOutput {
        amount,
        foreign_currency: input_foreign_currency.trim().to_string(),
        home_currency: input_home_currency.trim().to_string(),
    }
}
