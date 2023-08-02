mod get_data;
mod structs;
mod utils;

use colored::*;
use std::io;
use structs::CommandLineOutput;
use utils::ACCEPTED_CURRENCIES;

fn main() {
    let arguments = read_command_line();

    // make an http get request
    // parse data
    // oiut put the result
    println!("{:?}", arguments);
    println!("{:?}", ACCEPTED_CURRENCIES);
}

fn read_command_line() -> CommandLineOutput {
    let mut input_amount = String::new();
    let mut input_home_currency = String::new();
    let mut input_foreign_currency = String::new();
    loop {
        println!("Enter the exchange amount: ");
        input_amount.clear();
        match io::stdin().read_line(&mut input_amount) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to read input: {}. Please try again!", err);
                continue;
            }
        }

        match input_amount.trim().parse::<f64>() {
            Ok(value) => {
                // Store the amount in a local variable
                let amount = value;

                println!("Enter the home currency: ");
                match io::stdin().read_line(&mut input_home_currency) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Failed to read home currency input: {}", err);
                        continue; // Retry the loop for home_currency
                    }
                }

                println!("Enter the foreign currency: ");
                match io::stdin().read_line(&mut input_foreign_currency) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Failed to read home foreign input: {}", err);
                        continue; // Retry the loop for foreign_currency
                    }
                }

                // All inputs are successfully obtained, break the loop and return the result
                break CommandLineOutput {
                    amount,
                    foreign_currency: input_foreign_currency.trim().to_string(),
                    home_currency: input_home_currency.trim().to_string(),
                };
            }
            Err(_) => {
                println!("Could not parse amount into numeric value. Please try again!");
            }
        }
    }
}
