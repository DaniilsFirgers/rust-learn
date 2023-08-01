mod structs;

use colored::*;
use std::env;
use std::process;
use structs::CommandLineOutput;

const VALID_NUM_OF_ARGS: i32 = 3;

fn main() {
    let arguments = read_command_line();
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

    let amount = args[1].parse::<f64>().unwrap();
    let first_currency = args[2].parse::<f64>().unwrap();
    let second_currency = args[3].parse::<f64>().unwrap();

    CommandLineOutput {
        amount,
        foreign_currency: first_currency,
        home_currency: second_currency,
    }
}
