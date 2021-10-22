#!/usr/bin/python3

import sys

# Checks that the argument length is correct
if len ( sys.argv ) > 1 and len ( sys.argv ) != 3:
    
    # Incorrect argument length
    print ( "Incorrect argument length: Expected 2 arguments but was given {}".format ( len ( sys.argv ) - 1 ) )
    exit ()

"""
CHALLENGE 5:

Encrypts a given string with repeated XOR
"""

# XORs two bytes
def xor ( x, y ):
    return bytes ( a ^ b for a, b in zip ( x, y ) )

def main ( plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", key = "ICE" ):

    plaintext = bytes ( plaintext, "utf-8" )
    key = bytes ( key, "utf-8" )

    # Extends the key to the plaintext's length
    key = ( key * ( len ( plaintext ) + 4 ) )[0: len ( plaintext )]

    # Performs XOR operation
    return hex ( int.from_bytes ( xor ( plaintext, key ), "big" ) )[2:]

if len ( sys.argv ) > 1:
    print ( main ( sys.argv[1], sys.argv[2] ) )

else: 
    print ( main () )