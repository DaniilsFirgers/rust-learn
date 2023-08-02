mod structs;

use colored::*;
use std::env;
use std::process;
use structs::CommandLineOutput;

const VALID_NUM_OF_ARGS: i32 = 3;

fn main() {
    let arguments = read_command_line();
    println!("{:?}", arguments)
}

fn read_command_line() -> CommandLineOutput {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len() - 1;

    if args_len as i32 != VALID_NUM_OF_ARGS {
        eprintln!(
            "Provided {} arguments instead of {}",
            args_len.to_string().red(),
            VALID_NUM_OF_ARGS.to_string().green()
        );
        process::exit(1);
    }

    let amount = match args[1].parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid amount provided. Please enter a valid number.");
            process::exit(1);
        }
    };
    let first_currency = args[2].clone();
    let second_currency = args[3].clone();

    CommandLineOutput {
        amount,
        foreign_currency: first_currency,
        home_currency: second_currency,
    }
}
