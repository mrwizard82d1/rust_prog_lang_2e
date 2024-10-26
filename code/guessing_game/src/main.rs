// Import the standard I/O library
use std::io;

fn main() {
    // Print instructions to the user
    println!("Guess the number!");
    println!("Please input your guess.");

    // Initialize the user's guess
    let mut guess = String::new();

    // Read the guess from the user
    io::stdin()
        .read_line(&mut guess)
        // Expect to read a value; otherwise, an error occurred
        .expect("Failed to read line");

    // Print out the user's guess
    println!("You guessed: {}", guess);
}
