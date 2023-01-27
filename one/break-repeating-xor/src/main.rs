use std::fs::read_to_string;
use clap::{ Command, Arg };
use base64::Base64;
use xor::{ Xor, XorResult };

const MAX_KEY_LEN: usize = 40;

fn main () -> XorResult<()> {

    // Sets CLI options
    let matches = Command::new ( "Challenge Six - Breaking Repeating XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "Bruteforces a multiple-byte XOR ciphertext" )
        .arg ( Arg::new ( "encode" )
            .short( 'C' )
            .long ( "ciphertext" )
            .help ( "The ciphertext that should be bruteforced" )
        )
        .arg ( Arg::new ( "path" )
            .short( 'P' )
            .long ( "path" )
            .help ( "The path containing the ciphertext" )
        )
        .arg ( Arg::new ( "encoding" )
            .short ( 'E' )
            .long ( "encoding" )
            .require_equals ( true )
            .value_parser (["hex", "base64"])
            .help ( "The ciphertext encoding" )
        )
        .get_matches ();

    let encrypted_data = matches.get_one::<String> ( "ciphertext" );
    let filepath = matches.get_one::<String> ( "path" );
    let encoding = matches.get_one::<String> ( "encoding" )
        .expect ( "The --encoding option is required!" );

    let mut ciphertext = vec![];
    if let Some ( encrypted_data ) = encrypted_data {
        if encoding == "hex" {
            ciphertext = hex::decode ( encrypted_data.as_bytes ().to_vec () )?;
        } else if encoding == "base64" {
            ciphertext = Base64::decode ( encrypted_data.as_bytes ().to_vec () ).expect ( "Failed to decode Base64!" );
        }
    } else if let Some ( filepath ) = filepath {
        let encrypted_data = read_to_string ( filepath )?;
        if encoding == "hex" {
            ciphertext = hex::decode ( encrypted_data.as_bytes ().to_vec () )?;
        } else if encoding == "base64" {
            ciphertext = Base64::decode ( encrypted_data.as_bytes ().to_vec () ).expect ( "Failed to decode Base64!" );
        }
    } else {
        panic! ( "No ciphertext was provided!" )
    }

    // Decrypts the data
    let probable_length = Xor::estimate_key_length ( &ciphertext, MAX_KEY_LEN )?;
    let blocks = Xor::block_ciphertext ( &ciphertext, probable_length );
    let mut blocks = Xor::transpose_blocks ( &blocks, probable_length );
    let blocks = Xor::bruteforce_blocks ( &mut blocks );

    // Reorders bytes into a single vector
    let mut output: Vec<u8> = vec![];
    for x in 0..ciphertext.len () {
        let y = x % probable_length;
        let z = x / probable_length; // ???
        output.push ( blocks[y][z] );
    }

    println!("{:?}", String::from_utf8_lossy(&output[..]));

    Ok (())
}
