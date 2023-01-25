use xor::{ Xor, XorResult };
use clap::{ Command, arg };

fn main () -> XorResult<()> {

    // Sets CLI options
    let matches = Command::new ( "Challenge Two - XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "XOR's two hex values" )
        .arg ( arg! ( -L --lvalue <VALUE> "The hex lvalue" ).required ( true ) )
        .arg ( arg! ( -R --rvalue <VALUE> "The hex rvalue" ).required ( true ) )
        .get_matches ();

    let lvalue = hex::decode ( matches.get_one::<String> ("lvalue")
        .expect ( "The --lvalue option is required!" ) )?;

    let rvalue = hex::decode ( matches.get_one::<String> ( "rvalue" )
        .expect ( "The --rvalue option is required!" ) )?;

    let xor: Vec<u8> = Xor::hex ( lvalue, rvalue )?;

    println!("XOR hex output: {:?}", hex::encode ( xor.clone () ) );
    println!("XOR String output: {:?}", String::from_utf8 ( xor ).unwrap ());

    Ok (())
}
