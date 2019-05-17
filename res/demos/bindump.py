#!/usr/bin/env python3

"""
very simple binary dump script to form a readable ch8 file.
"""

import struct
import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file", help="input file with chip8 hexadecimal opcodes")
    parser.add_argument("output_file", help="output file to dump chip8 binary code into")
    args = parser.parse_args()

    with open(args.input_file, "r") as f:
        lines = f.readlines()
    
    wordlist = []
    for line in lines:
        # discard comments, by '#'.
        line = line[:line.find('#')]
        line.strip()
        
        words = line.split()
        if (len(words) > 0):
            word = int(words[0], 16)
            wordlist.append(word)
    
    print(wordlist)
    with open(args.output_file, "wb") as f:
        buff = bytearray(len(wordlist) * 2) 
        
        offset = 0
        for word in wordlist:
            struct.pack_into('>h', buff, offset, word)
            offset += 2

        f.write(buff)
        print("Wrote {} to {}".format(buff, args.output_file))
