use crate::errors::*;

pub struct Xor {}
 
impl Xor {

    pub fn hex ( mut lvalue: String, mut rvalue: String ) -> XorResult<Vec<u8>> {

        let lvalue = hex::decode ( &mut lvalue )?;
        let rvalue = hex::decode ( &mut rvalue )?;

        let xor: Vec<u8> = lvalue 
            .iter ()
            .zip ( rvalue.iter () )
            .map (|(&lvalue, &rvalue)| lvalue ^ rvalue )
            .collect ();

        Ok ( xor )
    }
}