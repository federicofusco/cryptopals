use std::fs::read_to_string;

// use bitvec::prelude::*;
use clap::{ Command, arg };
use xor::XorResult;

const MAX_KEY_LEN: usize = 40;

fn main () -> XorResult<()> {

    // Sets CLI options
    let matches = Command::new ( "Challenge Six - Breaking Repeating XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "Base64 decodes and bruteforces a given file" )
        .arg ( arg! ( -F --file <VALUE> "The file path which contains the ciphertext" ).required ( true ) )
        .get_matches ();

    let filepath = matches.get_one::<String> ( "file" )
        .expect ( "The --file option is required!" );

    // Reads and base64 decodes the given file
    let base64 = read_to_string ( filepath )?;

    // Loops through the "possible" key lengths
    for keysize in 0..MAX_KEY_LEN {

        // Step 3
        // let first_batch = 
    }

    Ok (())
}
