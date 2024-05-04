# Secret Message Sharing

This is a simple command-line application written in Rust that implements Shamir's Secret Sharing algorithm. It allows users to split a secret message into multiple shares and later reconstruct the original secret message from a subset of those shares.

## Features

- Split a secret message into multiple shares
- Reconstruct the original secret message from a subset of shares
- Input validation for the threshold value and share count
- Error handling for invalid shares or insufficient number of shares

## Requirements

- Rust compiler (version 1.46 or later)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository: git clone [https://github.com/niket-singh/sms.git](https://github.com/niket-singh/SMS)

2. Change to the project directory: cd sms

3. Build the project: cargo build --release

## Usage

1. Run the application: cargo run --release

2. Follow the prompts in the command-line interface:
   - Enter the secret message
   - Enter the threshold value (minimum number of shares required)
   - Enter the number of shares to generate
   - The application will display the generated shares

3. To reconstruct the secret message:
   - Enter the shares (at least the threshold number of shares)
   - Enter an empty line to finish
   - The application will display the reconstructed secret message, or an error if the reconstruction fails

## Example

Welcome to the Secret Message Sharing application!

Enter the secret message: My Secret Message

Enter the threshold value (minimum number of shares required): 3

Enter the number of shares to generate: 5

Generated shares:

Share 1: 1,77,121,32,83,101,99,114,101,116,32,77,101,115,115,97,103,101,65

Share 2: 2,192,88,169,245,178,30,169,239,40,29,23,225,180,82,207,232,190,201

Share 3: 3,160,70,102,65,252,119,147,200,227,105,136,170,173,124,201,46,26,194

Share 4: 4,42,133,38,211,90,233,63,212,142,219,3,231,11,87,206,172,129,159

Share 5: 5,183,40,208,60,91,146,175,151,209,225,208,80,46,23,158,230,15,39

Enter the shares (at least 3 shares) to reconstruct the secret message:

1,77,121,32,83,101,99,114,101,116,32,77,101,115,115,97,103,101,65

2,192,88,169,245,178,30,169,239,40,29,23,225,180,82,207,232,190,201

3,160,70,102,65,252,119,147,200,227,105,136,170,173,124,201,46,26,194

Reconstructed secret message: My Secret Message

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

This application uses the `shamirsecretsharing` crate, which provides an implementation of Shamir's Secret Sharing algorithm in Rust.

- [shamirsecretsharing](https://github.com/rust-crypto/shamirsecretsharing)

   
