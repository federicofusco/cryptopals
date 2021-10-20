#!/usr/bin/python3

import sys

# Checks that the argument length is correct
if len ( sys.argv ) != 2:
    
    # Incorrect argument length
    print ( "Incorrect argument length: Expected 1 arguments but was given {}".format ( len ( sys.argv ) - 1 ) )
    exit ()

# Checks that the input is a valid hexadecimal string
for x in sys.argv[1]:

    if x not in "0123456789abcdef":

        # Invalid hex string
        print ( "Invalid input: Expected valid hex string as first argument" )
        exit ()

"""
This challenge consists of converting a HEX encoded
string into a BASE64 encoded string
"""

def main ( input ):

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
    input_bitstring = ""
    output = ""

    # Converts the bytes into a bitstring and concatenates them
    for x in input_bytes:

        byte = "0" + bin ( x )[2:]

        while len ( byte ) < 8:

            byte = "0" + byte

        # concatenates byte
        input_bitstring += byte   

    # Converts the bytes into 6-bit values which can then be
    # Convered into ASCII characters based on the Base64 table
    for x in range ( int ( len ( input_bitstring ) / 6 ) ):

        y = x * 6

        value = "{}{}{}{}{}{}".format ( input_bitstring[y], input_bitstring[y + 1], input_bitstring[y + 2], input_bitstring[y + 3], input_bitstring[y + 4], input_bitstring[y + 5] )

        output += base64[int ( value, 2 )]

    return output

print ( main ( sys.argv[1] ) )