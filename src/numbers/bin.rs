use std::process;

use regex::Regex;

use crate::{hex::hex_from_dec, oct::oct_from_dec, Args};

const BIN: &str = "01";

pub fn handle_args(args: Args) {
    let re = Regex::new(r"^[0-1]+$").unwrap();

    if !re.is_match(&args.input) {
        eprintln!("Invalid digit found in string");
        process::exit(1);
    }

    let num = match u64::from_str_radix(&args.input, 2) {
        Ok(number) => number,
        Err(e) => panic!("Failed to parse binary string: {}", e),
    };

    println!("Octal: {}", oct_from_dec(num));
    println!("Decimal: {}", num);
    println!("Hexadecimal: {}", hex_from_dec(num));
}

pub fn bin_from_dec(input: u64) -> String {
    let mut string = String::new();

    let mut copy = input;

    while copy != 0 {
        let res = copy % 2;

        match BIN.chars().nth(res as usize) {
            Some(c) => string.push(c),
            None => {
                panic!("Inaccessible from bin");
            }
        }

        copy /= 2;
    }

    string.push_str("b0");
    string = string.chars().rev().collect::<String>();

    string
}
