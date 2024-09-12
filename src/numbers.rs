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

    if base != NumberBase::Binary {
        println!("Binary: {}", bin_from_dec(number));
    }
    if base != NumberBase::Octal {
        println!("Octal: {}", oct_from_dec(number));
    }
    if base != NumberBase::Decimal {
        println!("Decimal: {}", number);
    }
    if base != NumberBase::Hexadecimal {
        println!("Hexadecimal: {}", hex_from_dec(number));
    }
}
