use std::{ fs::File, io::{ BufRead, BufReader } };
use xor::{ Xor, XorResult };
use clap::{ Command, Arg, ArgAction };
 
fn main () -> XorResult<()> {

    // Sets CLI options
    let matches = Command::new ( "Challenge Four - Detecting Single Byte XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "Detects and bruteforces a list of XOR ciphertexts" )
        .arg ( Arg::new ( "path" ) 
            .short ( 'P' )
            .long ( "path" )
            .required ( true )
            .help ( "The path to the file containing the ciphertexts" )
        )
        .arg ( Arg::new ( "hex" )
            .short ( 'H' )
            .long ( "hex" )
            .action ( ArgAction::SetTrue )
            .help ( "Whether or not the ciphertexts are in a hexadecimal representation" )
        )
        .arg ( Arg::new ( "output" )
            .short ( 'O' )
            .long ( "output" )
            .require_equals ( true )
            .value_parser (["hex", "utf"])
            .help ( "The output format which should be returned" )
        )
        .get_matches ();

    let path = matches.get_one::<String> ("path")
        .expect ( "The --path option is required!" );
    let hex = matches.get_flag ( "hex" );
    let output = matches.get_one::<String> ( "output" )
        .expect ( "The --output option is required!" );

    // Attempts to open the given path
    let file = File::open ( path )?;
    let reader = BufReader::new ( file );
    let ciphertexts = reader.lines ();

    // Bruteforces the ciphertext
    let (probability, plaintext) = Xor::single_byte_bruteforce_list ( ciphertexts, hex )?;

    // Displays the plaintext
    if output == "hex" {
        println!( "Output ({probability}% certainty): {:?}", hex::encode ( plaintext ) ); 
    } else if output == "utf" {
        println!( "Output ({probability}% certainty): {:?}", String::from_utf8_lossy ( &plaintext[..] ) );
    }

    Ok (())
}
