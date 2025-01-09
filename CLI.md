## cipha-cli

### Introduction

The `cipha-cli` is a command-line interface (CLI) tool designed to encode and decode messages using various cryptographic ciphers. This tool leverages the `cipha-lib` crate, which provides implementations of several popular ciphers.

### Installation

To use the `cipha-cli`, you need to have Rust and the `cargo` package manager installed. It comes with the cipha crate
hence you can use
```bash
cargo add cipha
```

Here’s how you can build and install the CLI:

```bash
git clone https://github.com/jcharistech/cipha.git
cd cipha-cli
cargo build --release
cargo install --path .
```

### Usage

The `cipha-cli` supports two main subcommands: `encode` and `decode`. 
You can get a detailed overview of the available options and how to use them below.

#### Encode Subcommand

```bash
cipha encode --help
```


### Options

- `--cipher`: The cipher to use (e.g., rot13, caesar, reverse, gematria, vigenere, morse, atbash).
- `--message`: The message to encode.
- `--file`: Read the message from a file.
- `--shift`: Shift value for Caesar cipher (default: 3).
- `--key`: Key to encrypt by.
- `--output-file`: Output to a file instead of stdout.


#### Example Usage for Encode

```bash
cipha encode --cipher rot13 --message "Hello, World!"
cipha encode --cipher caesar --message "Hello, World!" --shift 3
cipha encode --cipher vigenere --message "Hello, World!" --key "LEMON"
cipha encode --cipher morse --message "Hello, World!"
cipha encode --cipher atbash --message "Hello, World!"
cipha encode --cipher gematria --message "Hello, World!"
cipha encode --cipher reverse --message "Hello, World!"
```

#### Decode Subcommand

```bash
cipha decode --help
```


#### Options

- `--cipher`: The cipher to use (e.g., rot13, caesar, reverse, gematria, vigenere, morse, atbash).
- `--message`: The message to decode.
- `--file`: Read the message from a file.
- `--shift`: Shift value for Caesar cipher (default: 3).
- `--key`: Key to decrypt by.
- `--output-file`: Output to a file instead of stdout.


#### Example Usage for Decode

```bash
cipha decode --cipher rot13 --message "Uryyb, Jbeyq!"
cipha decode --cipher caesar --message "Khoor, Zruog!" --shift 3
cipha decode --cipher vigenere --message "LXFOPVEFRNHR" --key "LEMON"
cipha decode --cipher morse --message ".... . .-.. .-.. ---"
cipha decode --cipher atbash --message "ZGGZXPZGWZDM"
cipha decode --cipher gematria --message "8 5 12 12 15 ,   23 15 18 12 4 !"
cipha decode --cipher reverse --message "!dlroW ,olleH"
```

### Supported Ciphers

- **ROT13**: A simple substitution cipher where each letter is shifted by 13 places.
  ```bash
  cipha encode --cipher rot13 --message "Hello, World!"
  cipha decode --cipher rot13 --message "Uryyb, Jbeyq!"
  ```

- **Caesar**: A substitution cipher where each letter is shifted by a specified number of places.
  ```bash
  cipha encode --cipher caesar --message "Hello, World!" --shift 3
  cipha decode --cipher caesar --message "Khoor, Zruog!" --shift 3
  ```

- **Reverse**: Reverses the input message.
  ```bash
  cipha encode --cipher reverse --message "Hello, World!"
  cipha decode --cipher reverse --message "!dlroW ,olleH"
  ```

- **Gematria**: Converts alphabetic characters to their corresponding numerical values and vice versa.
  ```bash
  cipha encode --cipher gematria --message "Hello, World!"
  cipha decode --cipher gematria --message "8 5 12 12 15 ,   23 15 18 12 4 !"
  ```

- **Vigenère**: A polyalphabetic substitution cipher that uses a keyword to shift each letter.
  ```bash
  cipha encode --cipher vigenere --message "Hello, World!" --key "LEMON"
  cipha decode --cipher vigenere --message "LXFOPVEFRNHR" --key "LEMON"
  ```

- **Morse**: Encodes and decodes messages into Morse code.
  ```bash
  cipha encode --cipher morse --message "Hello, World!"
  cipha decode --cipher morse --message ".... . .-.. .-.. ---"
  ```

- **Atbash**: A simple substitution cipher where each letter is replaced by its corresponding letter at the opposite end of the alphabet.
  ```bash
  cipha encode --cipher atbash --message "Hello, World!"
  cipha decode --cipher atbash --message "ZGGZXPZGWZDM"
  ```

### Retrieving Messages

The CLI can retrieve the message to be encoded or decoded from either a command-line argument (`--message`) or a file (`--file`).

```bash
cipha encode --cipher rot13 --message "Hello, World!"
cipha encode --cipher rot13 --file path/to/message.txt
```

### Output

By default, the output is written to `stdout`. You can specify an output file using the `--output-file` option.

```bash
cipha encode --cipher rot13 --message "Hello, World!" --output-file output.txt
```



## Conclusion

The `cipha-cli` provides a versatile and easy-to-use interface for encoding and decoding messages using various cryptographic ciphers. By following the examples and using the provided options, you can effectively utilize this tool in your cryptographic tasks.