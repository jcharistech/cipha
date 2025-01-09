//! This is the main module for the cipha-lib crate.
//! It contains various cipher and crypto functions.

pub mod ciphers;

use std::collections::HashMap;

/// Applies the ROT13 cipher to the input message.
///
/// # Examples
///
/// ```rust
/// use cipha_lib::rot13;
/// let input = "Hello, World!".to_string();
/// let output = rot13(input);
/// assert_eq!(output, "Uryyb, Jbeyq!");
/// ```
// Implement the ROT13 function
pub fn rot13(message: String) -> String {
    message.chars().map(|c| {
        match c {
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            _ => c,
        }
    }).collect()
}

/// Applies the Caesar cipher to the input message with the given shift.
///
/// # Examples
///
/// ```rust
/// use cipha_lib::caesar_cipher;
/// let input = "Hello, World!".to_string();
/// let shift = 3;
/// let output = caesar_cipher(input, shift);
/// assert_eq!(output, "Khoor, Zruog!");
/// ```
// Implement the Caesar Cipher function
pub fn caesar_cipher(message: String, shift: u8) -> String {
    message.chars().map(|c| {
        match c {
            'a'..='z' => (((c as u8 - b'a' + shift) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + shift) % 26) + b'A') as char,
            _ => c,
        }
    }).collect()
}


/// Reverses the input message.
///
/// This function takes a string slice and returns a new string with the characters in reverse order.
///
/// # Examples
///
/// ```rust
/// use cipha_lib::reverse_cipher;
/// let input = "Hello, World!";
/// let reversed = reverse_cipher(input);
/// assert_eq!(reversed, "!dlroW ,olleH");
/// ```
pub fn reverse_cipher(message: &str) -> String {
    message.chars().rev().collect()
}


/// Converts alphabetic characters in the input text to their corresponding numerical values.
///
/// This function iterates over each character in the input string. If the character is an alphabetic character, it converts it to its corresponding numerical value (A=1, B=2, ..., Z=26, a=1, b=2, ..., z=26) and appends it to the result string. Non-alphabetic characters are appended as is.
///
/// # Examples
///
/// ```rust
/// use cipha_lib::alpha2num;
/// let input = "Hello, World!";
/// let result = alpha2num(input);
/// assert_eq!(result, "8 5 12 12 15 ,   23 15 18 12 4 !");
/// ```
pub fn alpha2num(text: &str) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { 'A' as u32 } else { 'a' as u32 };
            let number = (c as u32 - base + 1) as u8;
            result.push_str(&number.to_string());
            result.push(' ');
        } else {
            result.push(c);
            result.push(' ');
        }
    }

    result.trim().to_string()
}


/// Converts numerical values in the input text back to their corresponding alphabetic characters.
///
/// This function parses the input string for numerical values separated by spaces. Each number is converted back to its corresponding alphabetic character (1=A, 2=B, ..., 26=Z, 27=A, 28=B, ..., 52=Z) and appended to the result string. Non-numerical characters are ignored.
///
/// # Examples
///
/// ```rust
/// use cipha_lib::num2alpha;
/// let input = "8 5 12 12 15 , 23 15 18 12 4 !";
/// let result = num2alpha(input);
/// assert_eq!(result, "helloworld");
/// ```
/// 
pub fn num2alpha(cipher_text: &str) -> String {
    let mut decrypted_text = String::new();
    let mut number_buffer = String::new();

    for char in cipher_text.chars() {
        if char.is_digit(10) {
            number_buffer.push(char);
        } else if char.is_whitespace() {
            if !number_buffer.is_empty() {
                let number: u32 = number_buffer.parse().unwrap();
                let char_code = if number <= 26 {
                    // Handle lowercase letters
                    'a' as u32 + number - 1
                } else if number <= 52 {
                    // Handle uppercase letters
                    'A' as u32 + number - 27
                } else {
                    // Ignore invalid numbers
                    continue;
                };
                decrypted_text.push(char::from_u32(char_code).unwrap());
                number_buffer.clear();
            }
        }
    }

    // Handle the last number if any
    if !number_buffer.is_empty() {
        let number: u32 = number_buffer.parse().unwrap();
        let char_code = if number <= 26 {
            'a' as u32 + number - 1
        } else if number <= 52 {
            'A' as u32 + number - 27
        } else {
            // Ignore invalid numbers
            return decrypted_text;
        };
        decrypted_text.push(char::from_u32(char_code).unwrap());
    }

    decrypted_text
}

/// Encrypts the input message using the Vigenère cipher with the given key.
///
/// # Examples
///
/// ```rust
/// use cipha_lib::vigenere_cipher;
/// let plaintext = "ATTACKATDAWN".to_string();
/// let key = "LEMON".to_string();
/// let ciphertext = vigenere_cipher(&plaintext, &key);
/// assert_eq!(ciphertext, "LXFOPVEFRNHR");
/// ```
pub fn vigenere_cipher(plaintext: &str, key: &str) -> String {
    let key: String = key.to_ascii_lowercase();
    let key_len = key.len();
    if key_len == 0 {
        return plaintext.to_string();
    }

    let mut index = 0;
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() {
                    b'a'
                } else {
                    b'A'
                };
                let shift = key.as_bytes()[index % key_len] as u8 - b'a';
                index += 1;
                // Modulo to keep character range
                (first + (c as u8 - first + shift) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_decipher(ciphertext: &str, key: &str) -> String {
    let key: String = key.to_ascii_lowercase();
    let key_len = key.len();
    if key_len == 0 {
        return ciphertext.to_string();
    }

    let mut index = 0;
    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() {
                    b'a'
                } else {
                    b'A'
                };
                let shift = key.as_bytes()[index % key_len] as u8 - b'a';
                index += 1;
                // Modulo to keep character range
               // Calculate the decrypted character code, avoiding overflow
               let decrypted_char_code = (c as u8 - first - shift + 26) % 26 + first;
           // Convert the decrypted character code back to a char
           decrypted_char_code as char
            } else {
                c
            }
        })
        .collect()
}


// Define the Morse code mapping

// Define the Morse code mapping
const MORSE_CODE_MAP: &[(&str, &str)] = &[
    ("A", ".-"), ("B", "-..."), ("C", "-.-."), ("D", "-.."), ("E", "."), ("F", "..-."),
    ("G", "--."), ("H", "...."), ("I", ".."), ("J", ".---"), ("K", "-.-"), ("L", ".-.."),
    ("M", "--"), ("N", "-."), ("O", "---"), ("P", ".--."), ("Q", "--.-"), ("R", ".-."),
    ("S", "..."), ("T", "-"), ("U", "..-"), ("V", "...-"), ("W", ".--"), ("X", "-..-"),
    ("Y", "-.--"), ("Z", "--.."),
    ("0", "-----"), ("1", ".----"), ("2", "..---"), ("3", "...--"), ("4", "....-"),
    ("5", "....."), ("6", "-...."), ("7", "--..."), ("8", "---.."), ("9", "----."),
    (" ", "/"), // Space separator
    (".", ".-.-.-"), (",", "--..--"), ("?", "..--.."), (";", "-.-.-."),
    (":", "---..."), ("-", "-....-"), ("/", "-..-."), ("'", ".----."),
    ("\"", ".-..-."),
    ("=", "-...-"), ("_", "..--.-"), ("+", ".-.-."), ("-", "-....-"),
    ("*", "-..-"), ("(", "-.--."), (")", "-.--.-"),
];

// Create a HashMap from the MORSE_CODE_MAP
pub fn create_morse_code_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (key, value) in MORSE_CODE_MAP {
        map.insert(key.to_string(), value.to_string());
    }
    map
}

// Create a reverse HashMap for decoding
pub fn create_reverse_morse_code_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (key, value) in MORSE_CODE_MAP {
        map.insert(value.to_string(), key.to_string());
    }
    map
}

/// Encodes the input message into Morse code.
///
/// # Examples
///
/// ```rust
/// use cipha_lib::morse_code_cipher;
/// let input = "HELLO".to_string();
/// let morse_code = morse_code_cipher(&input);
/// assert_eq!(morse_code, ".... . .-.. .-.. ---");
/// ```
// Function to encode a string into Morse code
pub fn morse_code_cipher(text: &str) -> String {
    let morse_code = create_morse_code_map();
    let mut encoded = String::new();

    for c in text.to_uppercase().chars() {
        if let Some(code) = morse_code.get(&c.to_string()) {
            encoded.push_str(code);
            encoded.push(' '); // Add space between characters
        }
    }

    encoded.trim().to_string() // Remove trailing space
}

// Function to decode Morse code into a string
pub fn morse_code_decipher(code: &str) -> String {
    let reverse_morse_code = create_reverse_morse_code_map();
    let mut decoded = String::new();
    let code_vec: Vec<&str> = code.split(' ').collect();

    for morse_char in code_vec {
        if let Some(character) = reverse_morse_code.get(morse_char) {
            decoded.push_str(character);
        }
    }

    decoded
}


/// Function to encipher a given plaintext using the Atbash cipher.
/// 
/// The Atbash cipher is a simple substitution cipher where each letter in the alphabet is replaced by its corresponding letter at the opposite end of the alphabet
/// # Parameters
/// - `plaintext`: The text to be encrypted.
///
/// # Returns
/// - The encrypted ciphertext.
pub fn atbash_cipher(plaintext: &str) -> String {
    plaintext.chars().map(|c| match c {
            'a'..='z' => ('a' as u8 + 25 - (c as u8 - 'a' as u8)) as char,
            'A'..='Z' => ('A' as u8 + 25 - (c as u8 - 'A' as u8)) as char,
            _ => c,
        }).collect()
}

/// Function to decipher a given ciphertext using the Atbash cipher.
/// 
/// The Atbash cipher is a simple substitution cipher where each letter in the alphabet is replaced by its corresponding letter at the opposite end of the alphabet
/// Note: The Atbash cipher is reciprocal, meaning the same function can be used for both encryption and decryption.
///
/// # Parameters
/// - `ciphertext`: The text to be decrypted.
///
/// # Returns
/// - The decrypted plaintext.
pub fn atbash_decipher(ciphertext: &str) -> String {
    atbash_cipher(ciphertext)
}


/// Function to encipher a given plaintext using the Rail Fence cipher.
    ///
    /// # Parameters
    /// - `plaintext`: The text to be encrypted.
    /// - `rails`: The number of rails in the fence.
    ///
    /// # Returns
    /// - The encrypted ciphertext.
    pub fn rail_fence_cipher(plaintext: &str, rails: usize) -> String {
        if rails == 1 || rails >= plaintext.len() {
            return plaintext.to_string();
        }

        let mut fence: Vec<String> = vec![String::new(); rails];
        let mut dir = 1; // Direction: 1 for downwards, -1 for upwards
        let mut rail = 0;

        for c in plaintext.chars() {
            fence[rail].push(c);
            if rail == 0 {
                dir = 1;
            } else if rail == rails - 1 {
                dir = -1;
            }
            rail = (rail as i32 + dir) as usize;
        }

        fence.iter().map(|s| s.clone()).collect::<Vec<String>>().join("")
    }

    /// Function to decipher a given ciphertext using the Rail Fence cipher.
    ///
    /// # Parameters
    /// - `ciphertext`: The text to be decrypted.
    /// - `rails`: The number of rails in the fence.
    ///
    /// # Returns
    /// - The decrypted plaintext.
    pub fn rail_fence_decipher(ciphertext: &str, rails: usize) -> String {
        if rails == 1 || rails >= ciphertext.len() {
            return ciphertext.to_string();
        }

        let mut fence: Vec<String> = vec![String::new(); rails];
        let mut dir = 1; // Direction: 1 for downwards, -1 for upwards
        let mut rail = 0;

        // Mark the positions of characters in the fence
        let mut positions: Vec<usize> = vec![0; rails];
        for _ in ciphertext.chars() {
            positions[rail] += 1;
            if rail == 0 {
                dir = 1;
            } else if rail == rails - 1 {
                dir = -1;
            }
            rail = (rail as i32 + dir) as usize;
        }

        // Fill the fence with characters from the ciphertext
        let mut ciphertext_chars = ciphertext.chars();
        for i in 0..rails {
            for _ in 0..positions[i] {
                fence[i].push(ciphertext_chars.next().unwrap());
            }
        }

        // Read the characters from the fence in the correct order
        let mut result = String::new();
        rail = 0;
        dir = 1;
        while result.len() < ciphertext.len() {
            if !fence[rail].is_empty() {
                result.push(fence[rail].remove(0));
            }
            if rail == 0 {
                dir = 1;
            } else if rail == rails - 1 {
                dir = -1;
            }
            rail = (rail as i32 + dir) as usize;
        }

        result
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13() {
        let input = "Hello, World!".to_string();
        let expected = "Uryyb, Jbeyq!".to_string();
        assert_eq!(rot13(input), expected);
    }

    #[test]
    fn test_caesar_cipher() {
        let input = "Hello, World!".to_string();
        let shift = 3;
        let expected = "Khoor, Zruog!".to_string();
        assert_eq!(caesar_cipher(input, shift), expected);
    }

    #[test]
    fn test_vigenere_cipher() {
        let plaintext = "ATTACKATDAWN".to_string();
        let key = "LEMON".to_string();
        let expected = "LXFOPVEFRNHR".to_string();
        assert_eq!(vigenere_cipher(&plaintext, &key), expected);
    }

    // #[test]
    // fn test_vigenere_decipher() {
    //     let ciphertext = "hello".to_string();
    //     let key = "some".to_string();
    //     let expected = "ATTACKATDAWN".to_string();
    //     assert_eq!(vigenere_decipher(&ciphertext, &key), expected);
    // }

    #[test]
    fn test_morse_code_cipher() {
        let input = "HELLO".to_string();
        let expected = ".... . .-.. .-.. ---".to_string();
        assert_eq!(morse_code_cipher(&input), expected);
    }

    #[test]
    fn test_morse_code_decipher() {
        let input = ".... . .-.. .-.. ---".to_string();
        let expected = "HELLO".to_string();
        assert_eq!(morse_code_decipher(&input), expected);
    }


    #[test]
    fn test_atbash_cipher() {
        let plaintext = "ATTACKATDAWN";
        let ciphertext = atbash_cipher(plaintext);
        assert_eq!(ciphertext, "ZGGZXPZGWZDM");

        let decrypted_text = atbash_decipher(&ciphertext);
        assert_eq!(decrypted_text, plaintext);
    }


    #[test]
    fn test_rail_fence_cipher() {
        let plaintext = "WEAREDISCOVEREDSAVEYOURSELF";
        let rails = 3;
        let ciphertext = rail_fence_cipher(plaintext, rails);
        assert_eq!(ciphertext, "WECRAOEERDSOEESVYUSLAIVDERF");

        let decrypted_text = rail_fence_decipher(&ciphertext, rails);
        assert_eq!(decrypted_text, plaintext);
    }
}