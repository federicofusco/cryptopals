#!/usr/bin/python3

import sys

# Checks that the argument length is correct
if len ( sys.argv ) != 2:
    
    # Incorrect argument length
    print ( "Incorrect argument length: Expected 1 arguments but was given {}".format ( len ( sys.argv ) - 1 ) )
    exit ()

"""
This challenge consists of brute-forcing all 
the possible signle byte keys which may have
been used to encrypt a series of payloads in 
a given file. The output should be the one 
payload which is actual English, while ignoring
the rest which are just noise
"""

def xor ( x, y ):
    return bytes ( a ^ b for a, b in zip ( x, y ) )

def find_key ( input ):

    # This dict contains the frequency of 
    # Each letter in the English alphabet
    frequency = {
        'a': 8.167,    'b': 1.492, 
        'c': 2.782,    'd': 4.253,
        'e': 12.702,   'f': 2.228,    
        'g': 2.015,    'h': 6.094,
        'i': 6.966,    'j': 0.153,    
        'k': 0.406,    'l': 4.025,
        'm': 2.204,    'n': 6.749,    
        'o': 7.507,    'p': 1.929,
        'q': 0.095,    'r': 5.987,    
        's': 6.327,    't': 9.056,
        'u': 2.758,    'v': 0.978,    
        'w': 2.361,    'x': 0.150,
        'y': 1.974,    'z': 0.074
    }

    # Converts the HEX encoded ciphertext into raw bytes
    ciphertext = bytes.fromhex ( input )

    keys = {}
    plaintexts = {}

    # Loops through each possible key
    # There are only 256 possibilities since we
    # Know that the key is only 1 byte long
    for x in range ( 256 ):

        # Extends the key byte to be the same length as the ciphertext
        key = bytes ([x]) * len ( ciphertext )

        # Performs XOR operation on ciphertext with the current key
        possible_plaintext = xor ( ciphertext, key )

        # Calculates the plaintext's probability
        probability = 0.0
        for y in possible_plaintext:

            char = chr ( y )
            
            if char.isalpha ():

                try:
                    if char == char.lower ():
                        probability += frequency[char]
                    else:

                        # Accounts for uppercase characters
                        probability += frequency[char.lower ()] * 0.75
                except: 

                    continue


        probability /= len ( ciphertext )

        for y in possible_plaintext:

            char = chr ( y )

            if not char.isalpha ():
                probability *= 0.90

        keys[key] = probability
        plaintexts[possible_plaintext] = probability

    return {
        "plaintext": max ( plaintexts, key=plaintexts.get ),
        "key": max ( keys, key=keys.get )[0],
        "probability": plaintexts[max ( plaintexts, key=plaintexts.get )]
    }

def main ( input_path ):

    input_file = open ( input_path, "r" )
    input = input_file.readlines ()

    # Strips the newline character
    plaintexts = {}
    for ciphertext in input:

        data = find_key ( ciphertext.strip () )

        plaintexts[ data["plaintext"] ] = data["probability"]

    print ( max ( plaintexts, key=plaintexts.get ) )

main ( sys.argv[1] )