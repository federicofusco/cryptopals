#!/usr/bin/python3

import sys

# Checks that the input is a valid hexadecimal string
if len ( sys.argv ) > 1:
    for x in sys.argv[1]:

        if x not in "0123456789abcdef":

            # Invalid hex string
            print ( "Invalid input: Expected valid hex string as first argument" )
            exit ()

"""
CHALLENGE 1:

Converts HEX encoded string into a BASE64 encoded string
"""

# Converts bytes into bitstrings
def byte_to_bits ( input ):

    output = ""

    for x in input:
        
        byte = "0" + bin ( x )[2:]

        while len ( byte ) < 8:

            byte = "0" + byte

        # concatenates byte
        output += byte
    
    return output

def main ( input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d" ):

    base64 = [
        "A", "B", "C", "D", "E", "F", "G", "H",
        "I", "J", "K", "L", "M", "N", "O", "P",
        "Q", "R", "S", "T", "U", "V", "W", "X",
        "Y", "Z", "a", "b", "c", "d", "e", "f",
        "g", "h", "i", "j", "k", "l", "m", "n",
        "o", "p", "q", "r", "s", "t", "u", "v",
        "w", "x", "y", "z", "0", "1", "2", "3", 
        "4", "5", "6", "7", "8", "9", "+", "/"
    ]

    input_bytes = bytes.fromhex ( input )
    input_bitstring = byte_to_bits ( input_bytes )
    output = ""

    # Converts the bytes into a bitstring and concatenates them 

    # Converts the bytes into 6-bit values which can then be
    # Convered into ASCII characters based on the Base64 table
    for x in range ( int ( len ( input_bitstring ) / 6 ) ):

        y = x * 6

        value = "{}{}{}{}{}{}".format ( input_bitstring[y], input_bitstring[y + 1], input_bitstring[y + 2], input_bitstring[y + 3], input_bitstring[y + 4], input_bitstring[y + 5] )

        output += base64[int ( value, 2 )]

    return output

if len ( sys.argv ) > 1:
    print ( main ( sys.argv[1] ) )

else:
    print ( main () )