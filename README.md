# shellcode-encoder

This script uses the base64 module to encode the shellcode into a base64 string. It then generates a list of random characters and shuffles them, and creates a dictionary of character substitutions based on the shuffled list.

The script then substitutes the characters in the encoded shellcode using the dictionary of substitutions, and prints the resulting encoded and substituted shellcode. This method of encoding the shellcode makes it difficult to reverse-engineer, as the characters in the encoded shellcode are not in their original positions.

To use this script, you would need to replace the shellcode variable with the actual shellcode you want to encode. You can then run the script and it will encode and substitute the shellcode, and print the resulting encoded and substituted shellcode. You can use the output of this script in your shellcode injection attacks, to make it more difficult for defenders to detect and reverse-engineer your shellcode.
