pub struct Xor {}
 
impl Xor {

    pub fn hex ( lvalue: Vec<u8>, rvalue: Vec<u8> ) -> Vec<u8> {
        let xor: Vec<u8> = lvalue 
            .iter ()
            .zip ( rvalue.iter () )
            .map (|(&lvalue, &rvalue)| lvalue ^ rvalue )
            .collect ();

        xor
    }

    pub fn repeated ( mut plaintext: Vec<u8>, key: Vec<u8> ) -> Vec<u8> {
        
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
}