use clap::{ Command, Arg, ArgAction };
use xor::{ Xor, XorResult };

fn main () -> XorResult<()> {

    // Sets CLI options
    let matches = Command::new ( "Challenge Three - Single Byte XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "Bruteforces an XORed ciphertext with a 1 byte key" )
        .arg ( Arg::new ( "ciphertext" ) 
            .short ( 'C' )
            .long ( "ciphertext" )
            .required ( true )
            .help ( "The value which will be bruteforced" )
        )
        .arg ( Arg::new ( "hex" )
            .short ( 'H' )
            .long ( "hex" )
            .action ( ArgAction::SetTrue )
            .help ( "Whether or not the ciphertext is a hexadecimal representation" )
        )
        .arg ( Arg::new ( "output" )
            .short ( 'O' )
            .long ( "output" )
            .require_equals ( true )
            .value_parser (["hex", "utf"])
            .help ( "The output format which should be returned" )
        )
        .get_matches ();

    let ciphertext = matches.get_one::<String> ("ciphertext")
        .expect ( "The --ciphertext option is required!" );
    let hex = matches.get_flag ( "hex" );
    let output = matches.get_one::<String> ( "output" )
        .expect ( "The --output option is required!" );

    // Bruteforces the ciphertext
    let (probability, plaintext): (u32, Vec<u8>);
    if hex {
        (probability, plaintext) = Xor::single_byte_bruteforce ( hex::decode ( ciphertext )? )?;
    } else {
        (probability, plaintext) = Xor::single_byte_bruteforce ( ciphertext.as_bytes ().to_vec () )?;
    }

    // Displays the plaintext
    if output == "hex" {
        println!( "Output ({probability}% certainty): {:?}", hex::encode ( plaintext ) );
    } else if output == "utf" {
        println!( "Output ({probability}% certainty): {:?}", String::from_utf8_lossy ( &plaintext[..] ) );
    }

    Ok (())
}
