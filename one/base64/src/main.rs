use base64::{ Base64, Base64Result };
use clap::{ Command, arg };

fn main () -> Base64Result<()> {
    
    // Sets CLI options
    let matches = Command::new ( "Challenge One - Base64")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "Converts hex values to base64" )
        .arg ( arg! ( -i --input <VALUE> "The hex input value" ).required ( true ) )
        .get_matches ();

    let hex = matches.get_one::<String> ("input")
        .expect ( "The --input option is required!" );

    // Converts from hex string
    let base64 = Base64::from_hex ( hex.to_string () )?; 
    println!("Base64 output: {:?}", base64); 

    return Ok (());
}
