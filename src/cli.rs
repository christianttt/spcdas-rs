use clap::Parser;
use std::num::ParseIntError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    pub input_file: String,

    #[arg(required = true)]
    pub output_file: String,

    #[arg(long, default_value = "0000", value_name = "ADDR")]
    pub load: String,

    #[arg(long, value_name = "ADDR")]
    pub pc: Option<String>,

    #[arg(long, default_value = "eof", value_name = "ADDR|eof")]
    pub stop: String,

    #[arg(long)]
    pub no_addr: bool,

    #[arg(long)]
    pub no_hex: bool,

    #[arg(long)]
    pub no_rel_resolve: bool,
}

pub fn parse_hex(s: &str) -> Result<u16, ParseIntError> {
    let s = s.trim_start_matches("0x");
    u16::from_str_radix(s, 16)
}
