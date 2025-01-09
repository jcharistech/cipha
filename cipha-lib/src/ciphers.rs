use std::collections::HashMap;
pub struct Rot13Cipher;

impl Rot13Cipher {
    pub fn new() -> Self {
        Rot13Cipher
    }

    pub fn encipher(&self, message: &str) -> String {
        message.chars().map(|c| {
            match c {
                'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
                'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
                _ => c,
            }
        }).collect()
    }

    pub fn decipher(&self, message: &str) -> String {
        self.encipher(message) // ROT13 is symmetric
    }
}

pub struct CaesarCipher {
    shift: u8,
}

impl CaesarCipher {
    pub fn new(shift: u8) -> Self {
        CaesarCipher { shift }
    }

    pub fn encipher(&self, message: &str) -> String {
        message.chars().map(|c| {
            match c {
                'a'..='z' => (((c as u8 - b'a' + self.shift) % 26) + b'a') as char,
                'A'..='Z' => (((c as u8 - b'A' + self.shift) % 26) + b'A') as char,
                _ => c,
            }
        }).collect()
    }

    pub fn decipher(&self, message: &str) -> String {
        let reverse_shift = 26 - self.shift;
        message.chars().map(|c| {
            match c {
                'a'..='z' => (((c as u8 - b'a' + reverse_shift) % 26) + b'a') as char,
                'A'..='Z' => (((c as u8 - b'A' + reverse_shift) % 26) + b'A') as char,
                _ => c,
            }
        }).collect()
    }
}

pub struct VigenereCipher {
    key: String,
}

impl VigenereCipher {
    pub fn new(key: &str) -> Self {
        VigenereCipher { key: key.to_ascii_lowercase() }
    }

    pub fn encipher(&self, plaintext: &str) -> String {
        let key_len = self.key.len();
        if key_len == 0 {
            return plaintext.to_string();
        }

        let mut index = 0;
        plaintext.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = self.key.as_bytes()[index % key_len] as u8 - b'a';
                index += 1;
                (first + (c as u8 - first + shift) % 26) as char
            } else {
                c
            }
        }).collect()
    }

    pub fn decipher(&self, ciphertext: &str) -> String {
        let key_len = self.key.len();
        if key_len == 0 {
            return ciphertext.to_string();
        }

        let mut index = 0;
        ciphertext.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = self.key.as_bytes()[index % key_len] as u8 - b'a';
                index += 1;
                (first + (c as u8 - first - shift + 26) % 26) as char
            } else {
                c
            }
        }).collect()
    }
}




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

pub struct MorseCode {
    morse_code_map: HashMap<String, String>,
    reverse_morse_code_map: HashMap<String, String>,
}

impl MorseCode {
    pub fn new() -> Self {
        let mut morse_code_map = HashMap::new();
        let mut reverse_morse_code_map = HashMap::new();

        for (key, value) in MORSE_CODE_MAP {
            morse_code_map.insert(key.to_string(), value.to_string());
            reverse_morse_code_map.insert(value.to_string(), key.to_string());
        }

        MorseCode {
            morse_code_map,
            reverse_morse_code_map,
        }
    }

    // Function to encode a string into Morse code
    pub fn encode(&self, text: &str) -> String {
        let mut encoded = String::new();

        for c in text.to_uppercase().chars() {
            if let Some(code) = self.morse_code_map.get(&c.to_string()) {
                encoded.push_str(code);
                encoded.push(' '); // Add space between characters
            }
        }

        encoded.trim().to_string() // Remove trailing space
    }

    // Function to decode Morse code into a string
    pub fn decode(&self, code: &str) -> String {
        let mut decoded = String::new();
        let code_vec: Vec<&str> = code.split(' ').collect();

        for morse_char in code_vec {
            if let Some(character) = self.reverse_morse_code_map.get(morse_char) {
                decoded.push_str(character);
            }
        }

        decoded
    }
}

pub struct AtbashCipher;

impl AtbashCipher {
    pub fn new() -> Self {
        AtbashCipher
    }

    // Function to encipher or decipher a string using the Atbash cipher
    pub fn transform(&self, text: &str) -> String {
        text.chars().map(|c| match c {
            'a'..='z' => ('a' as u8 + 25 - (c as u8 - 'a' as u8)) as char,
            'A'..='Z' => ('A' as u8 + 25 - (c as u8 - 'A' as u8)) as char,
            _ => c,
        }).collect()
    }

    // Function to encipher a string using the Atbash cipher
    pub fn encipher(&self, plaintext: &str) -> String {
        self.transform(plaintext)
    }

    // Function to decipher a string using the Atbash cipher
    pub fn decipher(&self, ciphertext: &str) -> String {
        self.transform(ciphertext)
    }
}


pub struct AlphaNumConverter;

impl AlphaNumConverter {
    pub fn new() -> Self {
        AlphaNumConverter
    }

    // Function to convert alphabetic characters to their corresponding numerical values
    pub fn alpha_to_num(&self, text: &str) -> String {
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

    // Function to convert numerical values back to their corresponding alphabetic characters
    pub fn num_to_alpha(&self, cipher_text: &str) -> String {
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
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13_encipher() {
        let rot13 = Rot13Cipher::new();
        let input = "Hello, World!";
        let expected = "Uryyb, Jbeyq!";
        assert_eq!(rot13.encipher(input), expected);
    }

    #[test]
    fn test_rot13_decipher() {
        let rot13 = Rot13Cipher::new();
        let input = "Uryyb, Jbeyq!";
        let expected = "Hello, World!";
        assert_eq!(rot13.decipher(input), expected);
    }

   

    #[test]
    fn test_caesar_encipher() {
        let caesar = CaesarCipher::new(3);
        let input = "Hello, World!";
        let expected = "Khoor, Zruog!";
        assert_eq!(caesar.encipher(input), expected);
    }

    #[test]
    fn test_caesar_decipher() {
        let caesar = CaesarCipher::new(3);
        let input = "Khoor, Zruog!";
        let expected = "Hello, World!";
        assert_eq!(caesar.decipher(input), expected);
    }

    #[test]
    fn test_vigenere_encipher() {
        let vigenere = VigenereCipher::new("LEMON");
        let input = "ATTACKATDAWN";
        let expected = "LXFOPVEFRNHR";
        assert_eq!(vigenere.encipher(input), expected);
    }

    // #[test]
    // fn test_vigenere_decipher() {
    //     let vigenere = VigenereCipher::new("LEMON");
    //     let input = "LXFOPVEFRNHR";
    //     let expected = "ATTACKATDAWN";
    //     assert_eq!(vigenere.decipher(input), expected);
    // }
    #[test]
    fn test_morse_code_encode() {
        let morse_code = MorseCode::new();
        let input = "HELLO";
        let expected = ".... . .-.. .-.. ---";
        assert_eq!(morse_code.encode(input), expected);
    }

    #[test]
    fn test_morse_code_decode() {
        let morse_code = MorseCode::new();
        let input = ".... . .-.. .-.. ---";
        let expected = "HELLO";
        assert_eq!(morse_code.decode(input), expected);
    }

    #[test]
    fn test_atbash_encipher() {
        let atbash = AtbashCipher::new();
        let input = "ATTACKATDAWN";
        let expected = "ZGGZXPZGWZDM";
        assert_eq!(atbash.encipher(input), expected);
    }

    #[test]
    fn test_atbash_decipher() {
        let atbash = AtbashCipher::new();
        let input = "ZGGZXPZGWZDM";
        let expected = "ATTACKATDAWN";
        assert_eq!(atbash.decipher(input), expected);
    }

    #[test]
    fn test_alpha_to_num() {
        let converter = AlphaNumConverter::new();
        let input = "Hello, World!";
        let expected = "8 5 12 12 15 ,   23 15 18 12 4 !";
        assert_eq!(converter.alpha_to_num(input), expected);
    }

    #[test]
    fn test_num_to_alpha() {
        let converter = AlphaNumConverter::new();
        let input = "8 5 12 12 15 , 23 15 18 12 4 !";
        let expected = "helloworld";
        assert_eq!(converter.num_to_alpha(input), expected);
    }
}
