#!/usr/bin/python3

import sys

# Checks that the argument length is correct
if len ( sys.argv ) != 3:
    
    # Incorrect argument length
    print ( "Incorrect argument length: Expected 2 arguments but was given {}".format ( len ( sys.argv ) - 1 ) )
    exit ()

"""
CHALLENGE 5
"""

def xor ( x, y ):
    return bytes ( a ^ b for a, b in zip ( x, y ) )

def main ( x, y ):

    plaintext = bytes ( x, "utf-8" )
    key = bytes ( y, "utf-8" )

    # Extends the key to the plaintext's length
    key = ( key * ( len ( plaintext ) + 4 ) )[0: len ( plaintext )]

    # Performs XOR operation
    return hex ( int.from_bytes ( xor ( plaintext, key ), "big" ) )

print ( main ( sys.argv[1], sys.argv[2] ) )