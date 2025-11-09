use std::io;

fn is_valid_greeting(input: &str) -> bool {
    // Check if an input contains at least one alphabetic character
    // and is not empty
    !input.is_empty() && input.chars().any(|c| c.is_alphabetic())
}

fn main() {
    println!("Greeting Program");
    println!("{}", "=".repeat(50));
    println!("Please enter a greeting: ");
    let mut user_greeting = String::new();
    io::stdin().read_line(&mut user_greeting).expect("Failed to read line");
    let user_greeting = user_greeting.trim().to_lowercase();

    // Validate input content
    if !is_valid_greeting(&user_greeting) {
        if user_greeting.is_empty() {
            println!("Error: Empty input. Please enter a valid greeting.");
        } else if user_greeting.chars().all(|c| c.is_numeric()) {
            println!("Error: Numbers only detected. Please enter a greeting with letters.");
        } else if user_greeting.chars().all(|c| !c.is_alphabetic()) {
            println!("Error: Symbols only detected. Please enter a valid greeting.");
        } else {
            println!("Error: Invalid input. Please enter a proper greeting.");
        }
        return;
    }

    // reply to validated greeting
    match user_greeting.as_str() {
        "good morning" | "hello" | "hi" | "hey" => {
            println!("Hello there! Welcome!");
        }
        "quit" | "bye" | "see you" | "exit" => {
            println!("Goodbye! See you later!");
        }
        _ => {
            println!("I don't understand that greeting.");
        }
    }
}