use xor::{ Xor, XorResult };
use clap::{ Command, Arg, ArgAction };

fn main () -> XorResult<()> {

    // Sets CLI options
    let matches = Command::new ( "Challenge Two - XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "XOR's two values" )
        .arg ( Arg::new ( "lvalue" ) 
            .short ( 'L' )
            .long ( "lvalue" )
            .required ( true )
            .help ( "The left value which will be XORed by the rvalue" )
        )
        .arg ( Arg::new ( "rvalue" )
            .short ( 'R' ) 
            .long ( "rvalue" )
            .required ( true )
            .help ( "The right value which will be XORed to the lvalue" )
        )
        .arg ( Arg::new ( "hex" )
            .short ( 'H' )
            .long ( "hex" )
            .action ( ArgAction::SetTrue )
            .help ( "Whether or not BOTH input values are in hexadecimal representations" )
        )
        .arg ( Arg::new ( "output" )
            .short ( 'O' )
            .long ( "output" )
            .require_equals ( true )
            .value_parser (["hex", "utf"])
            .help ( "The output format which should be returned" ) 
        )
        .get_matches ();

    let lvalue = matches.get_one::<String> ("lvalue")
        .expect ( "The --lvalue option is required!" );
    let rvalue = matches.get_one::<String> ( "rvalue" )
        .expect ( "The --rvalue option is required!" );
    let hex = matches.get_flag ( "hex" );
    let output = matches.get_one::<String> ( "output" )
        .expect ( "The --output option is required!" );

    // XORs the values
    let xor: Vec<u8>;
    if hex {
        xor = Xor::vec ( hex::decode ( lvalue )?, hex::decode ( rvalue )? )?;
    } else {
        xor = Xor::vec ( lvalue.as_bytes ().to_vec (), rvalue.as_bytes ().to_vec () )?;
    }

    // Prints the output
    if output == "hex" {
        println!("Output (hex): {:?}", hex::encode ( xor.clone () ) );
    } else if output == "utf" {
        println!("Output: {:?}", String::from_utf8 ( xor )? );
    }

    Ok (())
}
