use std::collections::btree_map::Entry::*;
use std::collections::BTreeMap;
use bitvec::prelude::*;
use crate::errors::*;

pub struct Xor {}
 
impl Xor {

    /// Calculates the probability of a string being english 
    /// by calculating its letter frequency
    pub fn probability ( input: &str ) -> f64 {
        
        let mut count: BTreeMap<char, f64> = BTreeMap::new ();
        for letter in input.chars () {
            match count.entry ( letter.to_ascii_uppercase () ) {
                Vacant ( entry ) => { entry.insert ( 1f64 ); }
                Occupied ( mut entry ) => *entry.get_mut () += 1f64,
            }
        }

        // Total number of characters in the given text
        let total = input.len() as f64;

        // Relative frequency of letters in the English language
        let mut english_frequencies: BTreeMap<char, f64> = BTreeMap::new ();
        english_frequencies.insert ( 'A', 0.0651738 );
        english_frequencies.insert ( 'B', 0.0124248 );
        english_frequencies.insert ( 'C', 0.0217339 );
        english_frequencies.insert ( 'D', 0.0349835 );
        english_frequencies.insert ( 'E', 0.1041442 );
        english_frequencies.insert ( 'F', 0.0197881 );
        english_frequencies.insert ( 'G', 0.0158610 );
        english_frequencies.insert ( 'H', 0.0492888 );
        english_frequencies.insert ( 'I', 0.0558094 );
        english_frequencies.insert ( 'J', 0.0009033 );
        english_frequencies.insert ( 'K', 0.0050529 );
        english_frequencies.insert ( 'L', 0.0331490 );
        english_frequencies.insert ( 'M', 0.0202124 );
        english_frequencies.insert ( 'N', 0.0564513 );
        english_frequencies.insert ( 'O', 0.0596302 );
        english_frequencies.insert ( 'P', 0.0137645 );
        english_frequencies.insert ( 'Q', 0.0008606 );
        english_frequencies.insert ( 'R', 0.0497563 );
        english_frequencies.insert ( 'S', 0.0515760 );
        english_frequencies.insert ( 'T', 0.0729357 );
        english_frequencies.insert ( 'U', 0.0225134 );
        english_frequencies.insert ( 'V', 0.0082903 );
        english_frequencies.insert ( 'W', 0.0171272 );
        english_frequencies.insert ( 'X', 0.0013692 );
        english_frequencies.insert ( 'Y', 0.0145984 );
        english_frequencies.insert ( 'Z', 0.0007836 );
        english_frequencies.insert ( ' ', 0.1918182 ); 

        // Update the counts to be the relative frequency of letters in the given text
        // and then calculate the Bhattacharyya coefficient as our score
        let mut score = 0.0;
        for letter in english_frequencies.keys () {
            match count.entry ( *letter ) {
                Vacant ( entry ) => {
                    entry.insert ( 0.0 );
                }
                Occupied ( mut entry ) => *entry.get_mut () /= total,
            }
   
            let partition_overlap = count[letter] * english_frequencies[letter];
            score += partition_overlap.sqrt ();
        }

        score
    }

    /// XORs two vectors of equal length
    pub fn vec ( lvalue: Vec<u8>, rvalue: Vec<u8> ) -> XorResult<Vec<u8>> {

        // Checks the vector lengths
        if lvalue.len () != rvalue.len () {
            return Err ( XorError::LengthNotEqual );
        }

        let xor: Vec<u8> = lvalue 
            .iter ()
            .zip ( rvalue.iter () )
            .map (|(&lvalue, &rvalue)| lvalue ^ rvalue )
            .collect ();

        Ok ( xor )
    }

    /// XORs two vectors of variable length
    /// 
    /// This works by padding the end of the first vector, a
    /// more "secure" (never use XOR for any level of security) 
    /// option is `variable_vec_secure`, which padds the vectors
    /// with random values 
    pub fn variable_vec ( mut plaintext: Vec<u8>, key: Vec<u8> ) -> Vec<u8> {
        
        // Pads the plaintext
        if plaintext.len () % key.len () != 0 {
            plaintext.extend_from_slice ( &vec![0; key.len () - plaintext.len () % key.len ()][..] );
        }

        // Loops through the plaintext
        let mut ciphertext: Vec<u8> = vec![];
        for i in 0..plaintext.len () {
            ciphertext.push ( plaintext[i] ^ key[i % key.len ()] );
        }

        ciphertext
    }

    /// Calculates the hamming distance between two bitvecs
    pub fn hamming_distance ( lvalue: BitVec<u8, Lsb0>, rvalue: BitVec<u8, Lsb0> ) -> XorResult<u32> {

        // Checks the vector lengths
        if lvalue.len () != rvalue.len () {
            return Err ( XorError::LengthNotEqual );
        }

        let mut distance: u32 = 0;
        for x in 0..lvalue.len () {
            if lvalue[x] != rvalue[x] {
                distance += 1;
            }
        }

        Ok ( distance )
    }
}