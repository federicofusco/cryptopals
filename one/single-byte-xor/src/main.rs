use clap::{ Command, arg };
use single_byte_xor::{ SBXor, SBXorResult };

fn main () -> SBXorResult<()> {

    // Sets CLI options
    let matches = Command::new ( "Challenge Three - Single Byte XOR")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "Bruteforces a hex input with a repeat 1 byte XOR operation" )
        .arg ( arg! ( -i --input <VALUE> "The hex input" ).required ( true ) )
        .get_matches ();

    let input = matches.get_one::<String> ("input")
        .expect ( "The --input option is required!" );

    let (probability, output) = SBXor::bruteforce ( input.to_string () )?;

    println!( "Output ({probability}% certainty): {output}" );

    Ok (())
}
