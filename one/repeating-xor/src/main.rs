use clap::{ Command, arg };
use xor::Xor;

fn main () {
    
    // Sets CLI options
    let matches = Command::new ( "Challenge Five - Repeated XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "XOR's an input with a given key" )
        .arg ( arg! ( -P --plaintext <VALUE> "The plaintext" ).required ( true ) )
        .arg ( arg! ( -K --key <VALUE> "The key" ).required ( true ) )
        .get_matches ();

    let mut plaintext = matches.get_one::<String> ("plaintext")
        .expect ( "The --plaintext option is required!" ).as_bytes ().to_vec ();

    let key = matches.get_one::<String> ( "key" )
        .expect ( "The --key option is required!" ).as_bytes ().to_vec ();

    let ciphertext = Xor::variable_vec ( &mut plaintext, &key );

    println!("XOR Output: {:?}", hex::encode ( ciphertext ) );
}