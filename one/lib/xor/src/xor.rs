use std::collections::btree_map::Entry::*;
use std::collections::BTreeMap;
use bitvec::prelude::*;
use crate::errors::*;
use std::io::BufRead;
use std::io::Lines;

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
    pub fn vec ( lvalue: &Vec<u8>, rvalue: &Vec<u8> ) -> XorResult<Vec<u8>> {

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
    pub fn variable_vec ( plaintext: &mut Vec<u8>, key: &Vec<u8> ) -> Vec<u8> {
        
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
    pub fn hamming_distance ( lvalue: &BitVec<u8, Lsb0>, rvalue: &BitVec<u8, Lsb0> ) -> XorResult<u32> {

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

    /// Brutforces an XORed input (single byte)
    /// 
    /// Returns a tuple containing the probability of it being correct (english)
    /// and the plaintext
    pub fn single_byte_bruteforce ( ciphertext: &mut Vec<u8> ) -> XorResult<(u32, Vec<u8>)> {
        let mut key = vec![0u8; ciphertext.len()];
        let mut tree: BTreeMap<u32, Vec<u8>> = BTreeMap::new ();

        // XORs every key
        for x in 0..u8::MAX {

            // Generates the key
            key.fill ( x );
        
            // XORs the input
            let xor = Self::variable_vec ( ciphertext, &key );
            let possible_plaintext = String::from_utf8_lossy ( &xor[..] ).to_string ();
            let probability = ( Self::probability ( possible_plaintext.clone ().as_str () ) * 100.0 ) as u32;
            tree.insert ( probability, xor );
        }

        // Gets the result with the highest probability
        let plaintext = tree
            .into_iter ()
            .max_by_key(|p| p.clone ().0 )
            .ok_or( XorError::ProbabilityCalc )?;

        Ok ( plaintext )
    }

    /// Bruteforces a series of single byte XOR ciphertexts and returns
    /// the ciphertext with the highest probability
    pub fn single_byte_bruteforce_list<T: BufRead> ( ciphertexts: Lines<T>, hex: bool ) -> XorResult<(u32, Vec<u8>)> {

        // Creates a BTreeMap to store the possible plaintexts
        let mut tree: BTreeMap<u32, Vec<u8>> = BTreeMap::new ();
    
        // Loops through the ciphertexts
        for ciphertext in ciphertexts {

            // Bruteforces the ciphertext
            let (probability, possible_plaintext): (u32, Vec<u8>);
            if hex {
                (probability, possible_plaintext) = Self::single_byte_bruteforce ( &mut hex::decode ( ciphertext? )? )?;
            } else {
                (probability, possible_plaintext) = Self::single_byte_bruteforce ( &mut ciphertext?.into () )?;
            }
            tree.insert ( probability, possible_plaintext );
        }

        let plaintext = tree 
            .into_iter () 
            .max_by_key(|p| p.clone ().0 )
            .ok_or ( XorError::ProbabilityCalc )?;

        Ok ( plaintext )
    }

    /// Divides the ciphertext into blocks of a given length
    /// The last block is not guaranteed to be the specified length
    pub fn block_ciphertext ( ciphertext: &Vec<u8>, block_length: usize ) -> Vec<Vec<u8>> {
        let mut blocks: Vec<Vec<u8>> = vec![vec![]; ciphertext.len () / block_length + 1];
        for x in (0..ciphertext.len ()).step_by ( block_length ) {
            if ciphertext.len () - x < block_length {
                blocks[(x as f64 / block_length as f64 ).ceil () as usize] = ciphertext[x..].to_vec ();
                break;
            }

            blocks[( x as f64 / block_length as f64).ceil () as usize] = ciphertext[x..x + block_length].to_vec ();
        }

        blocks
    }

    /// Transposes a ciphertext by a given key length
    /// This is useful for multiple byte XOR ciphertexts
    pub fn transpose_blocks ( blocks: &Vec<Vec<u8>>, length: usize ) -> Vec<Vec<u8>> {
        let mut transposed_blocks: Vec<Vec<u8>> = vec![vec![]; length];
        for x in blocks.iter () {
            for y in 0..x.len () {
                let _ = &mut transposed_blocks[y].push ( x[y] );
            }
        }

        transposed_blocks
    }

    /// Finds the most probable key length by calculating it's hamming distance
    pub fn estimate_key_length ( ciphertext: &Vec<u8>, max_length: usize ) -> XorResult<usize> {
        
        // Loops through the "possible" key lengths
        let mut key_lengths: BTreeMap<u32, usize> = BTreeMap::new ();
        for possible_length in 1..max_length {
            let sample_size = possible_length * 2;
            let n_samples = ciphertext.len () / sample_size - 1;

            // Calculates the length's score
            let mut length_score: f64 = 0.0;
            for x in 0..n_samples {
                length_score += Self::hamming_distance (
                    &BitVec::from_slice ( &ciphertext[x * sample_size..x * sample_size + possible_length] ), 
                    &BitVec::from_slice ( &ciphertext[x * sample_size + possible_length..x * sample_size + possible_length * 2] )
                )? as f64;
            }

            // Normalizes the score
            length_score /= possible_length as f64;
            length_score /= n_samples as f64;
            key_lengths.insert ( ( length_score * 10_000.0 ) as u32, possible_length );
        }

        let probable_length = *key_lengths.get ( 
            key_lengths
                .keys ()
                .next ()
                .map (|prob| prob)
                .ok_or ( XorError::ProbabilityCalc )?
            )
            .ok_or ( XorError::ProbabilityCalc )?;

        Ok ( probable_length ) 
    }

    /// Bruteforces a vector of single byte XOR ciphertexts
    pub fn bruteforce_blocks ( blocks: &mut Vec<Vec<u8>> ) -> Vec<Vec<u8>> {
        blocks  
            .into_iter ()
            .map (|mut block| Self::single_byte_bruteforce ( &mut block ).expect ( "Failed to bruteforce block!" ).1 )
            .collect ()
    }
}