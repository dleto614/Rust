use rand::Rng;
use std::io::{self, Write}; // Need Write for flush()

fn generate_random_number() -> u32 {
    let number = rand::rng().random_range(1..101);
    // number // Wtf is this? Like what? Apparently this is a valid return value.
    return number; 
}

fn get_input() -> String {
    let mut input: String = String::new();

    // Prompt on the same line
    print!("Please input your guess: ");
    
    match io::stdout().flush() {
        Ok(_) => {} // flushed successfully
        Err(_) => println!("Warning: failed to flush stdout"),
    }

    match io::stdin().read_line(&mut input) {
        Ok(_) => {} // Success: proceed
        Err(_) => {
            println!("Failed to read input. Try again.");
            return String::new(); // or loop to retry
        }
    };

    return input;
}

fn check_guess(guess: u32, number: u32) -> bool {
    if guess != number {
        return false;
    } else {
        return true;
    }
}

fn main() {
    let mut input: String = String::new();
    let guess: u32;

    println!("Guess the number!");

    let number = generate_random_number();

    println!("The secret number is: {}", number);

    input.clear(); // Added this so compiler would stop bitching that is never "used" as a useless warning.
    input = get_input();

    // Try parsing input.
    // Handle invalid input gracefully.
    guess = match input.trim().to_string().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return; // Exit early without panicking
        }
    };

    println!("You guessed: {}", guess);

    let check: bool = check_guess(guess, number);

    if check == true {
        println!("Guess was correct!");
    } else {
        println!("Guess was wrong!");
    }

}
