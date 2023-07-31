use std::env;
use std::process;

fn main() {
    read_command_line()
}

fn read_command_line() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Provided {} arguments instead of 3", args.len() - 1);
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
