use crate::errors::{ Base64Error, Base64Result };
use bitvec::prelude::*;

const BASE64_LOOKUP_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 
    'w', 'x', 'y', 'z', '0', '1', '2', '3', 
    '4', '5', '6', '7', '8', '9', '+', '/',
];

pub struct Base64 {} 

impl Base64 {

    /// Performs a lookup on the base64 lookup table O(n)
    fn lookup ( index: u8 ) -> Base64Result<u8> {
        Ok ( *BASE64_LOOKUP_TABLE.get ( index as usize ).ok_or ( Base64Error::LookupFailed )? as u8 )
    }

    // Performs a reverse lookup on the base64 lookup table O(n)
    fn reverse_lookup ( output: u8 ) -> Base64Result<u8> {
        if output == '=' as u8 {
            return Ok ( 65 );
        }

        Ok ( BASE64_LOOKUP_TABLE.iter ().position (|&x| x == output as char ).ok_or ( Base64Error::LookupFailed )?.try_into ()? )
    }

    /// Encodes a given vector of bytes to base64
    pub fn encode ( input: Vec<u8> ) -> Base64Result<Vec<u8>> {

        // Converts the input vector to a bit vector
        let mut input: BitVec<u8, Msb0> = BitVec::from_vec ( input );

        // Pads the bit vector
        let mut padding_length = 0;
        if input.len () % 6 != 0 {
            padding_length = ( 6 - input.len () % 6 ) / 2;
            for _ in 0..6 - input.len () % 6 {
                input.push ( false );
            } 
        }

        // Gets the sextets
        let mut output: Vec<u8> = vec![];
        for x in (0..input.len ()).step_by ( 6 ) { 
            let mut byte: u8 = 0;
            for y in x..x + 6 {
                byte = (byte << 1) | (input[y] as u8);
            }
            output.push ( Self::lookup ( byte )? );
        }

        // Appends the padding to the output
        let mut padding: Vec<u8> = vec!['=' as u8; padding_length];
        output.append ( &mut padding );

        Ok ( output )
    }


    /// Decodes a base64 encoded vector to a vector of bytes
    pub fn decode ( input: Vec<u8> ) -> Base64Result<Vec<u8>> {

        // Strips the padding (if it's present)
        let mut padding_length: usize = 0;
        if let Some ( padding_pos ) = input.iter ().position (|&x| x == '=' as u8 ) {
            padding_length = ( input.len () - padding_pos ) * 2;
        }

        // Performs a reverse lookup
        let input: Vec<u8> = input
            .into_iter ()
            .map (|x| Self::reverse_lookup ( x ).unwrap () )
            .collect ();

        // Converts the output vector to a bit vector
        let input: BitVec<u8, Msb0> = BitVec::from_vec ( input );

        // Removes extra bits
        let mut tmp_input: BitVec<u8, Msb0> = BitVec::EMPTY;
        for x in (0..input.len ()).step_by ( 8 ) {
            tmp_input.extend_from_bitslice ( &input[x + 2..x + 8] );
        }
        tmp_input = tmp_input[..tmp_input.len () - ( padding_length / 2 ) * 6].to_bitvec ();

        // Gets the octets
        let mut output: Vec<u8> = vec![];
        for x in (0..tmp_input.len () - padding_length).step_by ( 8 ) {
            let mut byte: u8 = 0;
            for y in x..x + 8 {
                byte = (byte << 1) | (tmp_input[y] as u8);
            }
            output.push ( byte );
        }

        Ok ( output )
    }

}