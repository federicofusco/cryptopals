#!/usr/bin/python3

import sys

# Checks that the input is a valid hexadecimal strings
if len ( sys.argv ) > 1:
    for x in sys.argv[1]:

        if x not in "0123456789abcdef":

            # Invalid hex string
            print ( "Invalid input: Expected valid hex string as first argument" )
            exit ()

"""
CHALLENGE 2:
XORs two HEX encoded strings
"""

def main ( input = "1c0111001f010100061a024b53535009181c", n = "686974207468652062756c6c277320657965" ):

    x = bytes.fromhex ( input )
    y = bytes.fromhex ( n )

    return hex ( int.from_bytes ( bytes ( a ^ b for a, b in zip ( x, y ) ), "big" ) )[2:]

if len ( sys.argv ) > 1:
    print ( main ( sys.argv[1] ) )

else:
    print ( main () )