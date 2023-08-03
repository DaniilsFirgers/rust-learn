mod get_data;
mod structs;
mod utils;

use colored::*;
use get_data::get_coversion_data;
use std::io;
use structs::CommandLineOutput;
use utils::ACCEPTED_CURRENCIES;

#[tokio::main]
async fn main() {
    let arguments = read_command_line();

    // make an http get request
    // parse data
    // oiut put the result
    println!("{:?}", arguments);
    println!("{:?}", ACCEPTED_CURRENCIES);
    get_coversion_data().await;
}

fn read_command_line() -> CommandLineOutput {
    let mut input_amount = String::new();
    let mut input_home_currency = String::new();
    let mut input_foreign_currency = String::new();
    let mut amount_numeric: f64 = 0.0;

    loop {
        println!("Enter the exchange amount: ");
        input_amount.clear();
        match io::stdin().read_line(&mut input_amount) {
            Ok(_) => match input_amount.trim().parse::<f64>() {
                Ok(value) => {
                    amount_numeric = value;
                }
                Err(err) => {
                    eprintln!("Failed to read home currency input: {}", err);
                    continue;
                }
            },
            Err(err) => {
                eprintln!("Failed to read input: {}. Please try again!", err);
                continue;
            }
        }
        break;
    }

    loop {
        println!("Enter the home currency: ");
        match io::stdin().read_line(&mut input_home_currency) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to read home currency input: {}", err);
                continue; // Retry the loop for home_currency
            }
        }
        break;
    }

    loop {
        println!("Enter the foreign currency: ");
        match io::stdin().read_line(&mut input_foreign_currency) {
            Ok(_) => {
                if input_foreign_currency
                    .trim()
                    .to_uppercase()
                    .eq(&input_home_currency.trim().to_uppercase())
                {
                    println!("Home currency is the same, please choose other foreign currency!");
                    continue;
                }
            }
            Err(err) => {
                eprintln!("Failed to read home foreign input: {}", err);
                continue; // Retry the loop for foreign_currency
            }
        }
        // All inputs are successfully obtained, break the loop and return the result
        break CommandLineOutput {
            amount: amount_numeric,
            foreign_currency: input_foreign_currency.trim().to_string(),
            home_currency: input_home_currency.trim().to_string(),
        };
    }
}
