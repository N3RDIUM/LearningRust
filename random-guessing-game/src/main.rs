// Use the use thing
// This is like import in py.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Print some random stuff
    println!("Welcome to the random guessing game! Hello World!");
    println!("Please enter your guess.");

    // Create the random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    
    // Define a new string to store the input
    let mut guess = String::new();   

    // Get input from the user
    io::stdin()
        .read_line(&mut guess)
        .expect("Whoops! Failed to read a line from stdin!");

    // Also convert the string to an int
    let guess: u32 = guess.trim().parse()
        .expect("No, I'm pretty sure that's not a number");

    // Using a println plaaceholder
    println!("I'm pretty sure you said: {guess}");

    // Compare the number with the user's guess
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Try a bigger number!"),
        Ordering::Equal => println!("Correct!"),
        Ordering::Greater => println!("Try a smaller number!")
    }
}
