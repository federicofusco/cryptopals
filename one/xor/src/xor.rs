use crate::errors::*;
pub struct Xor {}
 
impl Xor {

    pub fn hex ( lvalue: Vec<u8>, rvalue: Vec<u8> ) -> XorResult<Vec<u8>> {

        let xor: Vec<u8> = lvalue 
            .iter ()
            .zip ( rvalue.iter () )
            .map (|(&lvalue, &rvalue)| lvalue ^ rvalue )
            .collect ();

        Ok ( xor )
    }
}