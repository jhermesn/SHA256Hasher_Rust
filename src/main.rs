use std::io::{self, Write};
mod hasher;
use hasher::Sha256Hasher;

fn main() -> io::Result<()> {
    println!("Choose an operation:");
    println!("1) Calculate the SHA-256 hash of an input");
    println!("2) Validate an input against an expected hash");

    print!("Enter your choice (1 or 2): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    match choice {
        "1" => {
            print!("Enter the data to hash: ");
            io::stdout().flush()?;
            let mut data_input = String::new();
            io::stdin().read_line(&mut data_input)?;
            let data_input_trimmed = data_input.trim();

            let hash_result = Sha256Hasher::hash(data_input_trimmed);
            println!("SHA-256 Hash: {}", hash_result);
        }
        "2" => {
            print!("Enter the data to hash: ");
            io::stdout().flush()?;
            let mut data_input = String::new();
            io::stdin().read_line(&mut data_input)?;
            let data_input_trimmed = data_input.trim();

            print!("Enter the expected hash: ");
            io::stdout().flush()?;
            let mut expected_hash_input = String::new();
            io::stdin().read_line(&mut expected_hash_input)?;
            let expected_hash_trimmed = expected_hash_input.trim();

            let is_match = Sha256Hasher::verify(data_input_trimmed, expected_hash_trimmed);
            match is_match {
                false => {
                    println!("The hash and the data don't match.");
                }
                true => {
                    println!("The hash and the data match correctly.");
                }
            }
        }
        _ => println!("Invalid choice. Please restart the program and choose either 1 or 2."),
    }

    Ok(())
}
