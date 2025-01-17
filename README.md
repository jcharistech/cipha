# cipha
A simple CLI and Package for Classical Ciphers and Cryptography in Rust

[![Stable](https://img.shields.io/badge/docs-stable-blue.svg)](https://jcharistech.github.io/cipha/cipha/)
[![Dev](https://img.shields.io/badge/docs-dev-blue.svg)](https://jcharistech.github.io/cipha/cipha/)
[![](https://img.shields.io/crates/v/cipha.svg)](https://crates.io/crates/cipha)


## Introduction
`cipha` consist of two main part the `cipha` and  `cipha-cli`.


## Installation
To install `cipha` you can do so via cargo

```bash
cargo add cipha
```
Or simply put the following in your Cargo.toml.

```
[dependencies]
cipha = "0.1.0"
```

## Usage
The Function Based Approach
```rust
use cipha::utils::rot13;

fn main() {
  let input = "Hello, World!".to_string();
  let output = rot13(input);
  assert_eq!(output, "Uryyb, Jbeyq!");
}

```

Or via the Struct Based Approach
```rust
use cipha::ciphers::{Rot13Cipher};

fn main() {
    let r1 = Rot13Cipher::new();
    let encrypted = r1.encipher("Some string");
    let decrypted = r1.decipher(&encrypted);
    println!("Encrypted: {}, Decrypted: {}", encrypted, decrypted);
}
```



## cipha-lib

The `cipha-lib` crate provides a collection of cryptographic functions, including various ciphers and encoding schemes. It is designed to be easy to use and integrate into Rust projects.

### Functions
Below is a list of supported classical ciphers
+ rot13
+ caesar cipher
+ vigenere cipher
+ reverse
+ gematria
+ atbash
+ morse code

### ROT13 Cipher
The ROT13 cipher is a special case of the Caesar cipher where the shift is always 13.

#### Example
```rust
use cipha::utils::rot13;
let input = "Hello, World!".to_string();
let output = rot13(input);
assert_eq!(output, "Uryyb, Jbeyq!");
```

### Caesar Cipher
The Caesar cipher shifts each letter in the input message by a specified number of places.

#### Example
```rust
use cipha::utils::caesar_cipher;
let input = "Hello, World!".to_string();
let shift = 3;
let output = caesar_cipher(input, shift);
assert_eq!(output, "Khoor, Zruog!");
```

### Reverse Cipher
This function reverses the input message.

#### Example
```rust
use cipha::utils::reverse_cipher;
let input = "Hello, World!";
let reversed = reverse_cipher(input);
assert_eq!(reversed, "!dlroW ,olleH");
```

### Alpha to Num and Num to Alpha
These functions convert alphabetic characters to their corresponding numerical values and vice versa.
It is used to compute Gematria

#### Example
```rust
use cipha::utils::alpha2num;
let input = "Hello, World!";
let result = alpha2num(input);
assert_eq!(result, "8 5 12 12 15 ,   23 15 18 12 4 !");

use cipha::utils::num2alpha;
let input = "8 5 12 12 15 , 23 15 18 12 4 !";
let result = num2alpha(input);
assert_eq!(result, "Hello, World!");
```

### Vigenère Cipher
The Vigenère cipher uses a keyword to shift each letter in the input message.

#### Example
```rust
use cipha::utils::vigenere_cipher;
let plaintext = "ATTACKATDAWN".to_string();
let key = "LEMON".to_string();
let ciphertext = vigenere_cipher(&plaintext, &key);
assert_eq!(ciphertext, "LXFOPVEFRNHR");

use cipha::utils::vigenere_decipher;
let ciphertext = "LXFOPVEFRNHR".to_string();
let key = "LEMON".to_string();
let plaintext = vigenere_decipher(&ciphertext, &key);
assert_eq!(plaintext, "ATTACKATDAWN");
```

### Morse Code Cipher
This function encodes and decodes text into Morse code.

#### Example
```rust
use cipha::utils::morse_code_cipher;
let input = "HELLO".to_string();
let morse_code = morse_code_cipher(&input);
assert_eq!(morse_code, ".... . .-.. .-.. ---");

use cipha::utils::morse_code_decipher;
let input = ".... . .-.. .-.. ---".to_string();
let result = morse_code_decipher(&input);
assert_eq!(result, "HELLO");
```

### Atbash Cipher
The Atbash cipher is a simple substitution cipher where each letter is replaced by its corresponding letter at the opposite end of the alphabet.

#### Example
```rust
use cipha::utils::atbash_cipher;
let plaintext = "ATTACKATDAWN";
let ciphertext = atbash_cipher(plaintext);
assert_eq!(ciphertext, "ZGGZXPZGWZDM");

use cipha::utils::atbash_decipher;
let ciphertext = "ZGGZXPZGWZDM";
let plaintext = atbash_decipher(&ciphertext);
assert_eq!(plaintext, "ATTACKATDAWN");
```

### Rail Fence Cipher
This function encodes and decodes text using the Rail Fence cipher which is a transposition cipher.

#### Example
```rust
use cipha::utils::rail_fence_cipher;
let plaintext = "WEAREDISCOVEREDSAVEYOURSELF";
let rails = 3;
let ciphertext = rail_fence_cipher(plaintext, rails);
assert_eq!(ciphertext, "WECRAOEERDSOEESVYUSLAIVDERF");

use cipha::utils::rail_fence_decipher;
let ciphertext = "WECRAOEERDSOEESVYUSLAIVDERF";
let rails = 3;
let plaintext = rail_fence_decipher(&ciphertext, rails);
assert_eq!(plaintext, "WEAREDISCOVEREDSAVEYOURSELF");
```


### Conclusion

The `cipha-lib` crate provides a variety of cryptographic functions that can be easily integrated into your Rust projects. By following the examples and testing your code, you can ensure that your cryptographic operations are accurate and reliable.


### Cipha-lib: Struct Based Approach
+ We can also use the ciphers using a struct based approach inspired by the cipers package [ciphers.rs](https://crates.io/crates/ciphers)

```rust
use cipha::ciphers::*;

fn main() {
    let r1 = Rot13Cipher::new();
    let encrypted = r1.encipher("Some string");
    let decrypted = r1.decipher(&encrypted);
    println!("Encrypted: {}, Decrypted: {}", encrypted, decrypted);

    let c1 = CaesarCipher::new(3);
    let encrypted = c1.encipher("Some string");
    let decrypted = c1.decipher(&encrypted);
    println!("Encrypted: {}, Decrypted: {}", encrypted, decrypted);

    let v1 = VigenereCipher::new("LEMON");
    let encrypted = v1.encipher("ATTACKATDAWN");
    let decrypted = v1.decipher(&encrypted);
    println!("Encrypted: {}, Decrypted: {}", encrypted, decrypted);


    let morse_code = MorseCode::new();
    let input = "HELLO";
    let encoded = morse_code.encode(input);
    println!("Encoded: {}", encoded);

    let decoded = morse_code.decode(&encoded);
    println!("Decoded: {}", decoded);

      let converter = AlphaNumConverter::new();
    let input = "Hello, World!";
    let result = converter.alpha_to_num(input);
    println!("Alpha to Num: {}", result);

    let input = "8 5 12 12 15 , 23 15 18 12 4 !";
    let result = converter.num_to_alpha(input);
    println!("Num to Alpha: {}", result);

     let atbash = AtbashCipher::new();
    let plaintext = "ATTACKATDAWN";
    let ciphertext = atbash.encipher(plaintext);
    println!("Encrypted: {}", ciphertext);

    let decrypted_text = atbash.decipher(&ciphertext);
    println!("Decrypted: {}", decrypted_text);
}


```

## Cipha-cli
The `cipha-cli`  provides an interface to use all the ciphers from the command line. You can find more here [cipha-cli](CLI.md).
