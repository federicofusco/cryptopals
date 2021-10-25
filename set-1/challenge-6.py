#!/usr/bin/python3

import sys
import os
import base64

"""
CHALLENGE 6:

Breaks a BASE64 encoded string which has been encrypted with repeated
XOR and has a keysize between 1 and 40 bytes
"""


def xor(x: bytes, y: bytes) -> bytes:
    '''
    Perform single byte xor and repeating key xor.
    '''

    if len(y) > len(x):
        x, y = y, x

    if len(y) == 1:
        key = y[0]
        return bytearray(a ^ key for a in x)

    # Make y of same length as x by repeating it
    y *= 1 + (len(x) // len(y))
    y = y[:len(x)]

    return bytes(a ^ b for a, b in zip(x, y))

# Calculates the hamming distance between two bytes


def hamming_distance(x: bytes, y: bytes) -> int:
    '''
    Compute the hamming distance between two byte buffers.
    '''

    assert len(x) == len(y), 'Input byte arrays have mismatching lengths {} and {}'.format(len(x), len(y))

    distance = 0
    for z in xor(x, y):
        distance += bin(z).count("1")

    return distance


def calculate_probability(plaintext: str) -> float:
    '''
    Compute the probability of a text being written in plain english.
    '''

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

        char = chr(y)

        if char.isalpha():

            try:
                if char == char.lower():
                    probability += frequency[char]
                else:
                    # Accounts for uppercase characters
                    probability += frequency[char.lower()] * 0.75
            except:
                continue

    return probability


def find_key_size(ciphertext: bytes, length_range=(1, 40)) -> tuple[int, dict]:
    '''
    Estimate the best key size for repeating key XOR, based on hamming distance.

    Returns:
        - best_size: the estimated key size
        - probabilities: an ordered dictionary containing all key sizes' probabilities.
    '''

    probabilities = {}
    for key_size in range(*length_range):

        blocks = [ciphertext[x: x + key_size]
                  for x in range(0, len(ciphertext), key_size)]

        avg_distance = 0

        # Exclude the last block from computation,
        # because it may be shorter than keysize.
        for i in range(len(blocks) - 2):
            # Normalize distance by key_size
            avg_distance += hamming_distance(blocks[i], blocks[i+1]) / key_size

        avg_distance /= len(blocks) - 1

        probabilities[key_size] = avg_distance

    # Sort key lengths by probability.
    # This only works in Python 3.7+
    # learn more: https://stackoverflow.com/questions/613183
    probabilities = dict(sorted(probabilities.items(),
                         key=lambda item: item[1]))
    best_size = next(iter(probabilities))

    return best_size, probabilities


def find_single_byte_xor_key(ciphertext: bytes) -> int:
    '''
    Estimate the single byte key by computing the message's probability of it being written in english.
    '''

    best_score = -1
    best_key = -1

    # Loops through each possible key
    # There are only 256 possibilities since we
    # Know that the key is only 1 byte long
    for x in range(256):

        # Performs XOR operation on ciphertext with the current key
        possible_plaintext = xor(ciphertext, bytes([x]))
        probability = calculate_probability(possible_plaintext)

        for y in possible_plaintext:
            char = chr(y)

            if not char.isalpha():
                probability *= 0.90

        if probability > best_score:
            best_score = probability
            best_key = x

    return best_key


def decrypt_rotating_xor(ciphertext: bytes) -> tuple[bytes, bytes]:
    '''
    Decrypt a ciphertext encrypted with the repeating key XOR algorithm.

    Returns:
    - key
    - plaintext
    '''

    key = bytearray()
    best_key_size, _ = find_key_size(ciphertext)

    for x in range(best_key_size):
        block = [ciphertext[b] for b in range(x, len(ciphertext), best_key_size)]
        key_byte = find_single_byte_xor_key(block)
        key.append(int(key_byte))

    key = bytes(key)

    return key, xor(ciphertext, key)


def main(input_path=os.getcwd() + "/challenge-6.txt"):
    ciphertext = base64.b64decode(open(input_path, "r").read().encode("ascii"))
    key, plaintext = decrypt_rotating_xor(ciphertext)

    print('=' * 20)
    print('Key:', key)
    print('Key Length:', len(key))
    print('Input Length: ', len(ciphertext), '\tOutput Length: ', len(plaintext))
    print('=' * 20)
    print(plaintext.decode('utf-8'))



import unittest

class Challenge6Tests(unittest.TestCase):

    def test_xor(self):
        self.assertEqual(xor(b'\xf0\xf0\xf0\xf0', b'\x0f\x0f\x0f\x0f'), b'\xff\xff\xff\xff')
        str1 = b'lorem ipsum dolor sit amet'
        key = b'yellow submarine'

        bits = 0
        for b in xor(str1, str1):
            bits += bin(b).count("1")
        self.assertEqual(bits, 0)

        self.assertEqual(xor(xor(str1, key), key), str1)

    def test_hamming_distance(self):
        self.assertEqual(hamming_distance(b'this is a test', b'wokka wokka!!!'), 37)


if __name__ == "__main__":
    if len(sys.argv) > 1:
        main(sys.argv[1])
    else:
        main()

    unittest.main()