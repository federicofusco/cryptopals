use crate::errors::{ Base64Error, Base64Result };
use bitvec::prelude::*;

pub struct Base64 {} 

impl Base64 {

    /// Converts a bit vector a a vector of base64 encoded bytes
    fn bitvec_to_base64 ( bitvec: &mut BitVec<u8, Msb0> ) -> Base64Result<Vec<u8>> {

        // Pads the bitvector
        let pad_length = 6 - bitvec.len () % 6;
        if pad_length != 6 {
            for _ in 0..pad_length {
                bitvec.push ( false ); 
            }
        } 

        // Splits it into 6 bit groups
        let mut base64: Vec<u8> = vec![];
        for x in 0..bitvec.len () {
            
            // Skips iterations between groups of 6
            if x % 6 != 0 {
                continue;
            }
            
            let mut byte: u8 = 0;
            for y in 0..6 {
                let i = x + y;
                byte = ( byte << 1 ) | ( *bitvec.get(i).ok_or_else ( || Base64Error::Overflow )? as u8 ); 
            }
            base64.push ( byte );
        }

        Ok ( base64 )
    }

    /// Performs a lookup on the base64 lookup table with a given byte
    fn base64_lookup ( byte: u8 ) -> Base64Result<u8> {
        let table: [char; 64] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 
            'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 
            'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 
            'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 
            'w', 'x', 'y', 'z', '0', '1', '2', '3', 
            '4', '5', '6', '7', '8', '9', '+', '/',
        ];

        let char = table.get ( byte as usize );
        if let Some ( value ) = char {
            
            // The table index is valid
            Ok ( *value as u8 ) 
        } else {

            // Invalid table index
            Err ( Base64Error::LookupFailed )
        }
    }

    /// Converts a hex string to a base64 encoded string 
    pub fn from_hex ( mut hex: String ) -> Base64Result<String> {

        // Converts the hex string into a bit vector
        let bytevec = hex::decode ( &mut hex )?; 
        let mut bitvec: BitVec::<_, Msb0> = BitVec::from_slice ( &bytevec[..] );
 
        // Splits the bitvec into groups of six
        let bytevec_base64: Vec<u8> = Self::bitvec_to_base64 ( &mut bitvec )?
            .into_iter ()
            .map (|i| Self::base64_lookup ( i ).expect ( "Invalid hex values!" ) )
            .collect ();

        Ok ( String::from_utf8 ( bytevec_base64 )?.to_string () ) 
    }
}