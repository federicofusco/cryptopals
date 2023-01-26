use std::collections::BTreeMap;
use std::io::{ Lines, BufRead };
use crate::errors::*;
use xor::Xor;

pub struct SBXor {}

impl SBXor {

    /// Brutforces a given hex string input
    pub fn bruteforce ( ciphertext: String ) -> SBXorResult<(u32, String)> {
        let ciphertext = hex::decode ( ciphertext )?;
        let mut key = vec![0u8; ciphertext.len()];
        let mut tree: BTreeMap<u32, String> = BTreeMap::new ();

        // XORs every key
        for x in 0..u8::MAX {

            // Generates the key
            key.fill ( x );
        
            // XORs the input
            let xor = Xor::variable_vec ( ciphertext.clone (), key.clone () );
            let possible_plaintext = String::from_utf8_lossy ( &xor[..] ).to_string ();
            let probability = ( Xor::probability ( possible_plaintext.clone ().as_str () ) * 100.0 ) as u32;
            tree.insert ( probability, possible_plaintext );
        }

        // Gets the result with the highest probability
        let plaintext = tree
            .into_iter ()
            .max_by_key(|p| p.clone ().0 )
            .ok_or( SBXorError::ProbabilityCalc )?;

        Ok ( plaintext )
    }

    /// Bruteforces a series of ciphertexts and return the ciphertext 
    /// with the highest probability
    pub fn detect_bruteforce<T: BufRead> ( ciphertexts: Lines<T> ) -> SBXorResult<(u32, String)> {

        // Creates a BTreeMap to store the possible plaintexts
        let mut tree: BTreeMap<u32, String> = BTreeMap::new ();
    
        // Loops through the ciphertexts
        for ciphertext in ciphertexts {

            // Bruteforces the ciphertext
            let (probability, possible_plaintext) = Self::bruteforce ( ciphertext?.into () )?;
            tree.insert ( probability, possible_plaintext );
        }

        let plaintext = tree 
            .into_iter () 
            .max_by_key(|p| p.clone ().0 )
            .ok_or ( SBXorError::ProbabilityCalc )?;

        Ok ( plaintext )
    }
}