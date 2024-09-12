use clap::Parser;

mod numbers;
use numbers::*;

#[derive(clap::Parser)]
enum NumBaserCli {
    #[command(name = "bin")]
    FromBin(Args),

    #[command(name = "oct")]
    FromOct(Args),

    #[command(name = "dec")]
    FromDec(Args),

    #[command(name = "hex")]
    FromHex(Args),
}

#[derive(clap::Args)]
pub struct Args {
    #[arg(value_name = "NUMBER")]
    input: String,
}

fn main() {
    match NumBaserCli::parse() {
        NumBaserCli::FromBin(args) => handle_args(args, NumberBase::Binary),
        NumBaserCli::FromOct(args) => handle_args(args, NumberBase::Octal),
        NumBaserCli::FromDec(args) => handle_args(args, NumberBase::Decimal),
        NumBaserCli::FromHex(args) => handle_args(args, NumberBase::Hexadecimal),
    }
}
