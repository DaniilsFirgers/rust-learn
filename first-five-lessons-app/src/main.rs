mod get_data;
mod structs;
mod utils;

use colored::*;

use get_data::get_coversion_data;
use get_data::parse_config;
use std::io;
use structs::CommandLineOutput;
use utils::ACCEPTED_CURRENCIES;

const PRECISION: f64 = 100.0;
#[tokio::main]
async fn main() {
    let config = parse_config();
    let arguments = read_command_line();

    // parse data
    // oiut put the result
    let conversion_result = get_coversion_data(config, &arguments.home_currency).await;

    if let Ok(conversion) = conversion_result {
        let exchange_rate = conversion.data[&arguments.foreign_currency];
        let exchange_amount = exchange_rate * arguments.amount;

        println!(
            "You can exchange {:.2} {} to {:.2} {}",
            (arguments.amount * PRECISION).round() / PRECISION,
            arguments.home_currency,
            (exchange_amount * PRECISION).round() / PRECISION,
            arguments.foreign_currency
        );
    } else {
        eprintln!("Error while fetching conversion data!");
    }
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
        input_home_currency.clear();
        match io::stdin().read_line(&mut input_home_currency) {
            Ok(_) => {
                let converted_curr = &&*input_home_currency.trim().to_uppercase();
                if !ACCEPTED_CURRENCIES.contains(converted_curr) {
                    println!(
                        "Please choose a currency from the list: {:#?}",
                        ACCEPTED_CURRENCIES
                    );
                    continue;
                }
            }
            Err(err) => {
                eprintln!("Failed to read home currency input: {}", err);
                continue; // Retry the loop for home_currency
            }
        }
        break;
    }

    loop {
        println!("Enter the foreign currency: ");
        input_foreign_currency.clear();
        match io::stdin().read_line(&mut input_foreign_currency) {
            Ok(_) => {
                let converted_curr = input_foreign_currency.trim().to_uppercase();

                if !ACCEPTED_CURRENCIES.contains(&&*converted_curr) {
                    println!(
                        "Please choose a currency from the list: {:#?}",
                        ACCEPTED_CURRENCIES
                    );
                    continue;
                }
                if converted_curr.eq(&input_home_currency.trim().to_uppercase()) {
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
            foreign_currency: input_foreign_currency.trim().to_uppercase(),
            home_currency: input_home_currency.trim().to_uppercase(),
        };
    }
}
