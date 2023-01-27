use base64::Base64;
use clap::{Arg, Command};
use xor::{Xor, XorResult};

fn main() -> XorResult<()> {
    // Sets CLI options
    let matches = Command::new("Challenge Two - XOR")
        .version("0.1.0")
        .author("Federico Fusco")
        .about("XOR's two values")
        .arg(
            Arg::new("lvalue")
                .short('L')
                .long("lvalue")
                .required(true)
                .help("The left value which will be XORed by the rvalue"),
        )
        .arg(
            Arg::new("rvalue")
                .short('R')
                .long("rvalue")
                .required(true)
                .help("The right value which will be XORed to the lvalue"),
        )
        .arg(
            Arg::new("encoding")
                .short('E')
                .long("encoding")
                .require_equals(true)
                .value_parser(["hex", "base64"])
                .help("The input value encodings"),
        )
        .arg(
            Arg::new("output")
                .short('O')
                .long("output")
                .require_equals(true)
                .value_parser(["hex", "base64", "utf"])
                .help("The output format which should be returned"),
        )
        .get_matches();

    let lvalue = matches
        .get_one::<String>("lvalue")
        .expect("The --lvalue option is required!");
    let rvalue = matches
        .get_one::<String>("rvalue")
        .expect("The --rvalue option is required!");
    let encoding = matches
        .get_one::<String>("encoding")
        .expect("The --encoding option is required!");
    let output = matches
        .get_one::<String>("output")
        .expect("The --output option is required!");

    // XORs the values
    let mut xor: Vec<u8> = vec![];
    if encoding == "hex" {
        xor = Xor::vec(&hex::decode(lvalue)?, &hex::decode(rvalue)?)?;
    } else if encoding == "base64" {
        xor = Xor::vec(
            &Base64::decode(lvalue.as_bytes().to_vec()).expect("Failed to decode lvalue!"),
            &Base64::decode(rvalue.as_bytes().to_vec()).expect("Failed to decode rvalue!"),
        )?;
    }

    // Prints the output
    if output == "hex" {
        println!("{}", String::from_utf8(hex::encode(xor).into())?);
    } else if output == "base64" {
        println!(
            "{}",
            String::from_utf8(Base64::encode(xor).expect("Failed to encode base64 output!"))?
        );
    } else if output == "utf" {
        println!("{}", String::from_utf8(xor)?);
    }

    Ok(())
}
