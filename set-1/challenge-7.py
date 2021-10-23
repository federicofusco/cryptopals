#!/usr/bin/python3

import sys 

"""
CHALLENGE 7:

Implement AES encryption with the ECB
block mode
"""

# Splits a given byte string into blocks
def split_into_blocks ( plaintext, byte_length = 16 ):

    # Checks if the last block needs to be padded
    if len ( plaintext ) % byte_length > 0:
        
        # Padds plaintext using PKCS#5 padding
        missing_bytes = 16 - ( len ( plaintext ) % byte_length )
        for x in range ( missing_bytes ):
            plaintext.append ( missing_bytes )

    blocks = [ plaintext [ x : x + byte_length ] for x in range ( 0, len ( plaintext ), byte_length ) ]

    return blocks

def main ( plaintext = "helo" ):

    # Converts the ASCII plaintext to raw bytes
    plaintext = bytearray ( plaintext, "utf-8" )

    # Splits plaintext into 128-bit (16-byte) blocks
    blocks = split_into_blocks ( plaintext )

    print ( blocks )

if len ( sys.argv ) > 1:
    main ( sys.argv[1] )

else:
    main ()