use structopt::StructOpt;
use std::fs::File;
use std::io::{Write, Read, stdout};
extern crate cipha_lib; 
use cipha_lib::{alpha2num, atbash_cipher, atbash_decipher, caesar_cipher, morse_code_cipher, morse_code_decipher, num2alpha, reverse_cipher, rot13, vigenere_cipher, vigenere_decipher};

/// A simple CLI for ciphers and crypto.
///
/// This CLI supports various ciphers such as ROT13, Caesar, Vigenere, and more.
/// It can encode or decode messages using these ciphers.
#[derive(Debug, StructOpt)]
#[structopt(name = "cipha", about = "A simple CLI for ciphers and crypto")]
struct Cli {
    /// The subcommand to execute.
    #[structopt(subcommand)]
    cmd: Command,
    /// Output to a file instead of stdout.
    #[structopt(short, long, help = "Output to a file instead of stdout")]
    output_file: Option<String>,
}

/// Subcommands for the CLI.
///
/// These subcommands allow you to either encode or decode a message using different ciphers.
#[derive(Debug, StructOpt)]
enum Command {
    /// Encode a message using a cipher.
    ///
    /// # Options
    ///
    /// - `--cipher`: The cipher to use (e.g., rot13, caesar).
    /// - `--message`: The message to encode.
    /// - `--file`: Read the message from a file.
    /// - `--shift`: Shift value for Caesar cipher (default: 3).
    /// - `--key`: Key to encrypt by.
    #[structopt(name = "encode", about = "Encode a message using a cipher")]
    Encode {
        /// The cipher to use.
        #[structopt(short, long, help = "The cipher to use (e.g., rot13, caesar)")]
        cipher: String,
        /// The message to encode.
        #[structopt(short = "m", long = "message", help = "The message to encode")]
        message: Option<String>,
        /// Read the message from a file.
        #[structopt(short = "f", long = "file", help = "Read the message from a file")]
        file: Option<String>,
        /// Shift value for Caesar cipher.
        #[structopt(short, long, help = "Shift value for Caesar cipher (default: 3)")]
        shift: Option<u8>,
        /// Key to encrypt by.
        #[structopt(short = "k", long = "key", help = "Key to encrypt by")]
        key: Option<String>,
    },
    /// Decode a message using a cipher.
    ///
    /// # Options
    ///
    /// - `--cipher`: The cipher to use (e.g., rot13, caesar, reverse, gematria).
    /// - `--message`: The message to decode.
    /// - `--file`: Read the message from a file.
    /// - `--shift`: Shift value for Caesar cipher (default: 3).
    /// - `--key`: Key to decrypt by.
    #[structopt(name = "decode", about = "Decode a message using a cipher")]
    Decode {
        /// The cipher to use.
        #[structopt(short, long, help = "The cipher to use (e.g., rot13, caesar, reverse, gematria)")]
        cipher: String,
        /// The message to decode.
        #[structopt(short = "m", long = "message", help = "The message to decode")]
        message: Option<String>,
        /// Read the message from a file.
        #[structopt(short = "f", long = "file", help = "Read the message from a file")]
        file: Option<String>,
        /// Shift value for Caesar cipher.
        #[structopt(short, long, help = "Shift value for Caesar cipher (default: 3)")]
        shift: Option<u8>,
        /// Key to decrypt by.
        #[structopt(short = "k", long = "key", help = "Key to decrypt by")]
        key: Option<String>,
    },
}


fn main() {
    let opt = Cli::from_args();
    // add cmd_type to avoid trying to borrow opt.cmd after its fields have been moved out,
    let (cipher, message, shift, key, cmd_type) = match opt.cmd {
        Command::Encode { cipher, message, file, shift, key } => {
            let msg = get_message(message, file).expect("Could not get message");
            (cipher, msg, shift, key, "encode")
        }
        Command::Decode { cipher, message, file, shift, key } => {
            let msg = get_message(message, file).expect("Could not get message");
            (cipher, msg, shift, key, "decode")
        }
    };
    
    let result = match cmd_type {
        "encode" => encode_message(cipher, message, shift, key),
        "decode" => decode_message(cipher, message, shift, key),
        _ => "Unsupported command".to_string(),
    };

    let mut output: Box<dyn Write> = if let Some(file_path) = &opt.output_file {
        Box::new(File::create(file_path).expect("Could not create file"))
    } else {
        Box::new(stdout())
    };

    writeln!(output, "{}", result).expect("Could not write to output");
}

/// Retrieves the message from either the command-line argument or a file.
///
/// # Errors
///
/// Returns an error if neither `--message` nor `--file` is provided.
fn get_message(message: Option<String>, file: Option<String>) -> Result<String, String> {
    match (message, file) {
        (Some(msg), _) => Ok(msg),
        (_, Some(file_path)) => {
            let mut file = File::open(file_path).map_err(|e| format!("Could not open file: {}", e))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(|e| format!("Could not read file: {}", e))?;
            Ok(contents)
        }
        (None, None) => Err("Either --message or --file must be provided".to_string()),
    }
}


/// Encodes a message using the specified cipher.
///
/// # Supported Ciphers
///
/// - `rot13`: ROT13 cipher.
/// - `caesar`: Caesar cipher with the given shift.
/// - `reverse`: Reverses the message.
/// - `gematria`: Converts alphabetic characters to their corresponding numerical values.
/// - `vigenere`: Vigenere cipher with the given key.
/// - `morse`: Encodes the message into Morse code.
/// - `atbash`:  Atbash cipher the message.
fn encode_message(cipher: String, message: String, shift: Option<u8>, key: Option<String>) -> String {
    match cipher.as_str() {
        "rot13" => rot13(message),
        "caesar" => caesar_cipher(message, shift.unwrap_or(3)),
        "reverse" => reverse_cipher(&message),
        "gematria" => alpha2num(&message),
        "vigenere" => vigenere_cipher(&message, &key.unwrap_or("".to_string())),
        "morse" => morse_code_cipher(&message),
        "atbash" => atbash_cipher(&message),
        _ => "Unsupported cipher".to_string(),
    }
}


/// Decodes a message using the specified cipher.
///
/// # Supported Ciphers
///
/// - `rot13`: ROT13 cipher.
/// - `caesar`: Caesar cipher with the given shift.
/// - `reverse`: Reverses the message.
/// - `gematria`: Converts numerical values back to alphabetic characters.
/// - `vigenere`: Vigenere cipher with the given key.
/// - `morse`: Decodes Morse code back to the original message.
/// - `atbash`: Decodes Atbash cipher back to the original message.
fn decode_message(cipher: String, message: String, shift: Option<u8>, key: Option<String>) -> String {
    match cipher.as_str() {
        "rot13" => rot13(message),
        "caesar" => caesar_cipher(message, shift.unwrap_or(3) * 25), // Reverse shift for decryption
        "reverse" => reverse_cipher(&message),
        "gematria" => num2alpha(&message),
        "vigenere" => vigenere_decipher(&message, &key.unwrap_or("".to_string())),
        "morse" => morse_code_decipher(&message),
        "atbash" => atbash_decipher(&message),
        _ => "Unsupported cipher".to_string(),
    }
}