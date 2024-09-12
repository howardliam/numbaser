use std::process;

use regex::Regex;

use crate::{bin::bin_from_dec, hex::hex_from_dec, oct::oct_from_dec, Args};

pub fn handle_args(args: Args) {
    let re = Regex::new(r"^[0-9]+$").unwrap();

    if !re.is_match(&args.input) {
        eprintln!("Invalid digit found in string");
        process::exit(1);
    }

    let num = match u64::from_str_radix(&args.input, 10) {
        Ok(number) => number,
        Err(e) => panic!("Failed to parse decimal string: {}", e),
    };

    println!("Binary: {}", bin_from_dec(num));
    println!("Octal: {}", oct_from_dec(num));
    println!("Hexadecimal: {}", hex_from_dec(num));
}
