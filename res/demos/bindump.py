#!/usr/bin/env python3

"""
very simple binary dump script to form a readable ch8 file.
"""

import struct

if __name__ == "__main__":
    with open("adder.txt.ch8", "r") as f:
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
    with open("adder.ch8", "wb") as f:
        buff = bytearray(len(wordlist) * 2) 
        
        offset = 0
        for word in wordlist:
            struct.pack_into('>h', buff, offset, word)
            offset += 2

        f.write(buff)
        print("Wrote {} to adder.ch8".format(buff))
