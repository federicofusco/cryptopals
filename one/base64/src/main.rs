use base64::{ Base64, Base64Result };
use clap::{ Command, Arg, ArgAction };

fn main () -> Base64Result<()> {
    
    // Sets CLI options
    let matches = Command::new ( "Challenge One - Base64")
        .version ( "0.1.0" )
        .author ( "Federico Fusco" )
        .about ( "Converts hex values to base64" )
        .arg ( Arg::new ( "encode" )
            .short( 'E' )
            .long ( "encode" )
        )
        .arg ( Arg::new ( "decode" )
            .short( 'D' )
            .long ( "decode" )
        )
        .arg ( Arg::new ( "hex" )
            .short ( 'H' )
            .long ( "hex" )
            .action ( ArgAction::SetTrue )
        )
        .get_matches ();


    // Checks if the inputs are hex
    let hex = matches.get_flag ( "hex" );

    // Encodes a given value
    if let Some ( encode ) = matches.get_one::<String> ("encode") {

        // Base64 encodes the input
        let base64: Vec<u8>;
        if hex {
            base64 = Base64::encode ( hex::decode ( encode )? )?;
        } else {
            base64 = Base64::encode ( encode.as_bytes ().to_vec () )?;
        }

        println!("Encoded output: {:?}", String::from_utf8 ( base64 )? ); 
    }

    // Decodes a given value
    if let Some ( decode ) = matches.get_one::<String> ("decode") {

        // Base64 decodes the input
        let base64: Vec<u8>;
        if hex {
            base64 = Base64::decode ( hex::decode ( decode )? )?;
        } else {
            base64 = Base64::decode ( decode.as_bytes ().to_vec () )?;
        }

        println!("Decoded output: {:?}", String::from_utf8 ( base64 )? ); 
    }

    Ok (())
}
