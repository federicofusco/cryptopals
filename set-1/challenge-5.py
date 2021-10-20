#!/usr/bin/python3

import sys

# Checks that the argument length is correct
if len ( sys.argv ) != 2:
    
    # Incorrect argument length
    print ( "Incorrect argument length: Expected 1 arguments but was given {}".format ( len ( sys.argv ) - 1 ) )
    exit ()

"""
This challenge consists of performing an XOR operation
on a given HEX encoded string with a set number
"""

def xor ( x, y ):
    return bytes ( a ^ b for a, b in zip ( x, y ) )

def main ( input ):

    plaintext = bytes ( input, "utf-8" )
    key = bytes ( "ICE", "utf-8" )

    # Extends the key to the plaintext's length
    key = ( key * ( len ( plaintext ) + 4 ) )[0: len ( plaintext )]

    # Performs XOR operation
    return hex ( int.from_bytes ( xor ( plaintext, key ), "big" ) )

print ( main ( sys.argv[1] ) )