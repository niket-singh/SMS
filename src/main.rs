use std::io::{self, Write};
use shamirsecretsharing::*;

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_valid_input(prompt: &str, validate: Box<dyn Fn(u8) -> bool>, error_msg: &str) -> u8 {
    loop {
        let input = get_user_input(prompt);
        if let Ok(value) = input.parse::<u8>() {
            if validate(value) {
                return value;
            }
        }
        println!("{}", error_msg);
    }
}

fn main() {
    println!("Welcome to sms App");

    let secret = get_user_input("Enter the secret message: ");
    let threshold = get_valid_input("Enter the threshold value: ", Box::new(|v| v > 0), "Invalid threshold value between 1 and 255.");
    let share_count = get_valid_input("Enter the number of shares to generate: ", Box::new(move |v| v >= threshold && v <= 255), "Invalid share count");

    let shares = create_shares(secret.as_bytes(), threshold, share_count);
    println!("\nGenerated shares: ");
    shares.iter().enumerate().for_each(|(i, share)| println!("Share {} : {:?}", i+1, share));

    println!("\nEnter the shares (at least {} shares) to reconstruct the secret message: ", threshold);
    println!("Enter an empty line to finish");
    let mut input_shares = Vec::new();
    loop {
        let share = get_user_input("");
        if share.is_empty() {
            break;
        }
        input_shares.push(share);
    }

    let input_shares: Vec<Vec<u8>> = input_shares.iter().map(|s| s.as_bytes().to_vec()).collect();

    if let Ok(reconstructed_secret) = combine_shares(&input_shares) {
        if input_shares.len() >= threshold as usize {
            println!("\nReconstructed secret message: {:?}", reconstructed_secret);
        } else{
            println!("Insufficient shares");
        }
    } else {
        println!("Invalid shares");
    }
}
