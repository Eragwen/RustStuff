use std::env; // for args
use std::fs::File; // for file
use std::io::{BufRead, BufReader}; // for file reading
use sha2::{Sha256, Digest}; // for sha256
use std::process::exit; // for exit

// Function: main
fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments
    if args.len() != 2 {
        println!("Invalid amount of arguments");
        println!("Example: cargo run <sha256 hash>");
        exit(1);
    }

    // Get the desired hash to crack
    let wanted_hash: &String = &args[1];

    // Define the path to the password file
    let password_file: &str = "src/passwords.txt";

    // Initialize the number of attempts
    let mut attempts: i32 = 1;

    println!("Attempting to crack hash: {}!\n", wanted_hash);

    // Open the password file
    let password_list: File = File::open(password_file).expect("Failed to open the password file");
    
    // Create a buffered reader to read the password file line by line
    let reader: BufReader<File> = BufReader::new(password_list);

    // Iterate over each line in the password file
    for line in reader.lines() {
        let line: String = line.expect("Failed to read line"); // Get the current line
        let password: Vec<u8> = line.trim().to_owned().into_bytes(); // Convert the line to a password (Vec<u8>)

        // Hash the password using SHA-256
        let password_hash: String = format!("{:x}", Sha256::digest(&password));

        // Print the attempt number, the password, and its hash
        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).expect("Failed to convert bytes to string"), password_hash);

        // Check if the calculated hash matches the desired hash
        if &password_hash == wanted_hash {
            println!("Password hash found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&password).expect("Failed to convert bytes to string"), password_hash);
            exit(0); // Exit with success status code
        }
        attempts += 1; // Increment the number of attempts
    }

    // If the loop finishes without finding the password, print failure message
    println!("Password hash not found after {} attempts!", attempts);
}
