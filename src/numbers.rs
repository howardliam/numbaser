use std::process;

use bin::bin_from_dec;
use hex::hex_from_dec;
use oct::oct_from_dec;

use crate::Args;

pub mod bin;
pub mod hex;
pub mod oct;

#[derive(PartialEq, Eq)]
pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl NumberBase {
    pub fn to_integer(&self) -> u32 {
        match self {
            NumberBase::Binary => 2,
            NumberBase::Octal => 8,
            NumberBase::Decimal => 10,
            NumberBase::Hexadecimal => 16,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            NumberBase::Binary => "binary",
            NumberBase::Octal => "octal",
            NumberBase::Decimal => "decimal",
            NumberBase::Hexadecimal => "hexadecimal",
        }
    }
}

pub fn handle_args(args: Args, base: NumberBase) {
    let radix = base.to_integer();
    let number = match u64::from_str_radix(&args.input, radix) {
        Ok(num) => num,
        Err(err) => {
            eprintln!("Failed to parse `{}` as integer: {}", &args.input, err);
            process::exit(1);
        }
    };

    println!("Converting {} `{}`\n", base.to_str(), number);

    if base != NumberBase::Binary {
        println!("Binary: \t{}", bin_from_dec(number));
    }
    if base != NumberBase::Octal {
        println!("Octal: \t\t{}", oct_from_dec(number));
    }
    if base != NumberBase::Decimal {
        println!("Decimal: \t{}", number);
    }
    if base != NumberBase::Hexadecimal {
        println!("Hexadecimal: \t{}", hex_from_dec(number));
    }
}
