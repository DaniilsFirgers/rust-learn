use colored::*;
use std::env;
use std::process;

const VALID_NUM_OF_ARGS: i32 = 3;

fn main() {
    read_command_line()
}

fn read_command_line() {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len() - 1;

    if args_len != VALID_NUM_OF_ARGS.try_into().unwrap() {
        eprintln!(
            "Provided {} arguments instead of {}",
            args_len.to_string().red(),
            VALID_NUM_OF_ARGS.to_string().green()
        );
        process::exit(1);
    }

    let amount = &args[1];
    let first_currency = &args[2];
    let second_currency = &args[3];

    println!(
        "Amount  is {}, currency 1 is {}, currency 2 is {}",
        amount, first_currency, second_currency
    );
}
