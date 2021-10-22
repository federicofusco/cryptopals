#!/usr/bin/python3

import sys
import base64

# Checks that the argument length is correct
if len ( sys.argv ) != 2:
    
    # Incorrect argument length
    print ( "Incorrect argument length: Expected 1 argument but was given {}".format ( len ( sys.argv ) - 1 ) )
    exit ()

"""
CHALLENGE 6
"""

def xor ( x, y ):
    return bytes ( a ^ b for a, b in zip ( x, y ) )

def hamming_distance ( x, y ):    

    distance = 0
    for z in xor ( bytes ( x ), bytes ( y ) ):
        distance += bin ( z ).count ( "1" )

    return distance

def calculate_probability ( plaintext ):

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

    return probability

def find_key ( ciphertext ):

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

        probability = calculate_probability ( possible_plaintext )

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

    key = bytearray ()

    # Decodes the payload
    ciphertext = base64.b64decode ( open ( input_path, "r" ).read ().encode ( "ascii" ) )

    # Loops through each possible keysize and calculates their probability
    probability = {}
    for keysize in range ( 1, 40 ):

        # Breaks the ciphertext into blocks the size of the keysize
        blocks = [ ciphertext [ x : x + keysize ] for x in range ( 0, len ( ciphertext ), keysize ) ]

        # # Calculates the normalized hamming distance between the first 2 blocks
        # distance = hamming_distance ( blocks[0], blocks[1] ) / keysize

        distA = hamming_distance ( blocks[0], blocks[1] ) / keysize
        distB = hamming_distance ( blocks[1], blocks[2] ) / keysize

        distance = ( distA + distB ) / 2

        probability[keysize] = ( distance, blocks )

    probability = list ( { k: v for k, v in sorted ( probability.items (), key = lambda item: item[1] ) }.items () )[:10]

    # Loops through the 10 most probable keysizes
    plaintexts = {}
    keys = {}
    for keysize, prob in probability:

        decrypted_bytes = []

        plaintext = ""
        key = None

        for x in range ( keysize ):
            
            byte_at_x = bytearray ()

            for block in prob[1]:

                if not len ( block ) < keysize:
                    byte_at_x.append ( block[x] )

            data = find_key ( byte_at_x )

            decrypted_bytes.append ( data["plaintext"] )

        for x in range ( len ( decrypted_bytes[0] ) ):
            for y in range ( keysize ):
                plaintext += chr ( decrypted_bytes[y][x] )

        plaintexts[plaintext] = calculate_probability ( bytes ( plaintext, "utf-8" ) )


    return max ( plaintexts, key=plaintexts.get )


print ( main ( sys.argv[1] ) )