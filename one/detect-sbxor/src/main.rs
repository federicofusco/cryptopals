use std::{ fs::File, io::{ BufRead, BufReader } };
use single_byte_xor::{ SBXor, SBXorResult };
use clap::{ Command, arg };

fn main () -> SBXorResult<()> {
    
    // Sets CLI options
    let matches = Command::new ( "Challenge Four - Detecting SBX")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "XOR's two hex values" )
        .arg ( arg! ( -F --file <VALUE> "The file path which contains the ciphertexts" ).required ( true ) )
        .get_matches ();

    let filepath = matches.get_one::<String> ( "file" )
        .expect ( "The --file option is required!" );

    // Attempts to open the given path
    let file = File::open ( filepath ).unwrap ();
    let reader = BufReader::new ( file );
    let ciphertexts = reader.lines ();

    // Detects the ciphertext
    let (probability, plaintext) = SBXor::detect_bruteforce::<BufReader<File>> ( ciphertexts )?;    

    println!("Output ({probability}% certainty): {:?}", plaintext);

    Ok (())
}
