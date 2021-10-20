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

def byte_to_bits ( input ):

    output = ""

    for x in input:
        
        byte = "0" + bin ( x )[2:]

        while len ( byte ) < 8:

            byte = "0" + byte

        # concatenates byte
        output += byte
    
    return output


def main ( input ):

    n = "686974207468652062756c6c277320657965"
    a_bitstring = byte_to_bits ( bytes.fromhex ( input ) )
    b_bitstring = byte_to_bits ( bytes.fromhex ( n ) )
    output_bitstring = ""

    for x in range ( len ( a_bitstring ) ):

        if int ( a_bitstring[x] ) + int ( b_bitstring[x] ) == 1:

            output_bitstring += "1"

        else:

            output_bitstring += "0"

    return hex ( int ( output_bitstring, 2 ) )[2:]

print ( main ( sys.argv[1] ) )