use std::process;

use regex::Regex;

use crate::{bin::bin_from_dec, oct::oct_from_dec, Args};

const HEX: &str = "0123456789ABCDEF";

pub fn handle_args(args: Args) {
    let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();

    if !re.is_match(&args.input) {
        eprintln!("Invalid digit found in string");
        process::exit(1);
    }

    let num = match u64::from_str_radix(&args.input, 16) {
        Ok(number) => number,
        Err(e) => panic!("Failed to parse hexadecimal string: {}", e),
    };

    println!("Binary: {}", bin_from_dec(num));
    println!("Octal: {}", oct_from_dec(num));
    println!("Decimal: {}", num);
}

pub fn hex_from_dec(input: u64) -> String {
    let mut string = String::new();

    let mut copy = input;

    while copy > 0 {
        let res = copy % 16;

        match HEX.chars().nth(res as usize) {
            Some(c) => string.push(c),
            None => {
                panic!("Inaccessible from hex");
            }
        }

        copy /= 16;
    }

    string.push_str("x0");
    string = string.chars().rev().collect::<String>();

    string
}
