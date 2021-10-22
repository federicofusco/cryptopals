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
CHALLENGE 3:

Decrypts a message encrypted with a single
byte XOR
"""

# XORs two bytes
def xor ( x, y ):
    return bytes ( a ^ b for a, b in zip ( x, y ) )

# Calculates the probability that a given plaintext is 
# Written in English
def calculate_probability ( plaintext, ciphertext ):
    
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

    # Calculates the plaintext's probability
    probability = 0.0
    for y in plaintext:

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

    for y in plaintext:

        char = chr ( y )

        if not char.isalpha ():
            probability *= 0.90
    
    return probability

def main ( input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736" ):

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

        # Calculates the probability the the plaintext is correct
        probability = calculate_probability ( possible_plaintext, ciphertext )

        keys[key] = probability
        plaintexts[possible_plaintext] = probability

    return max ( plaintexts, key=plaintexts.get )

if len ( sys.argv ) > 1:
    print ( main ( sys.argv[1] ) )

else:
    print ( main () )