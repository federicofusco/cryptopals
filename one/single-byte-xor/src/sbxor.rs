use std::collections::BTreeMap;
use std::collections::btree_map::Entry::*;
use crate::errors::*;
use xor::Xor;

pub struct SBXor {}

impl SBXor {

    /// Calculates the probability of a string being english 
    /// by calculating its letter frequency
    fn probability ( input: &str ) -> f64 {
        
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

    /// Brutforces a given hex string input
    pub fn bruteforce ( input: String ) -> SBXorResult<(u32, String)> {
        let input = hex::decode ( input )?;
        let mut key = vec![0u8; input.len()];
        let mut tree: BTreeMap<u32, String> = BTreeMap::new ();

        // XORs every key
        for x in 0..u8::MAX {

            // Generates the key
            key.fill ( x );
        
            // XORs the input
            let xor = Xor::hex ( input.clone (), key.clone () )?;
            let utf8 = String::from_utf8_lossy ( &xor[..] ).to_string ();
            let prob = ( Self::probability ( utf8.clone ().as_str () ) * 100.0 ) as u32;
            tree.insert ( prob, utf8 );
        }

        // Gets the result with the highest probability
        let result = tree
            .into_iter ()
            .max_by_key(|p| p.clone ().0 )
            .ok_or( SBXorError::ProbabilityCalc )?;

        Ok ( result )
    }
}