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
        NumBaserCli::FromBin(args) => bin::handle_args(args),
        NumBaserCli::FromOct(args) => oct::handle_args(args),
        NumBaserCli::FromDec(args) => dec::handle_args(args),
        NumBaserCli::FromHex(args) => hex::handle_args(args),
    }
}
