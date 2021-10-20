#!/usr/bin/python3

import sys

# Checks that the argument length is correct
if len ( sys.argv ) != 2:
    
    # Incorrect argument length
    print ( "Incorrect argument length: Expected 1 arguments but was given {}".format ( len ( sys.argv ) - 1 ) )
    exit ()

# Checks that the inputs are both valid hexadecimal strings
for x in sys.argv[1]:

    if x not in "0123456789abcdef":

        # Invalid hex string
        print ( "Invalid input: Expected valid hex string as first argument" )
        exit ()

"""
This challenge consists of performing an XOR operation
on a given HEX encoded string with a set number
"""

def main ( input ):

    n = "686974207468652062756c6c277320657965"
    x = bytes.fromhex ( input )
    y = bytes.fromhex ( n )

    return bytes ( a ^ b for a, b in zip ( x, y ) )

print ( hex ( int.from_bytes ( main ( sys.argv[1] ), "big" ) )[2:] )