use std::process;

use regex::Regex;

use crate::{bin::bin_from_dec, hex::hex_from_dec, Args};

const OCT: &str = "01234567";

pub fn handle_args(args: Args) {
    let re = Regex::new(r"^[0-7]+$").unwrap();

    if !re.is_match(&args.input) {
        eprintln!("Invalid digit found in string");
        process::exit(1);
    }

    let num = match u64::from_str_radix(&args.input, 8) {
        Ok(number) => number,
        Err(e) => panic!("Failed to parse octal string: {}", e),
    };

    println!("Binary: {}", bin_from_dec(num));
    println!("Decimal: {}", num);
    println!("Hexadecimal: {}", hex_from_dec(num));
}

pub fn oct_from_dec(input: u64) -> String {
    let mut string = String::new();

    let mut copy = input;

    while copy > 0 {
        let res = copy % 8;

        match OCT.chars().nth(res as usize) {
            Some(c) => string.push(c),
            None => {
                panic!("Inaccessible from oct");
            }
        }

        copy /= 8;
    }

    string.push_str("o0");
    string = string.chars().rev().collect::<String>();

    string
}
