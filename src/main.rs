// Import the I/O library  
use std::io; 

// Import random
use rand::Rng;

fn main() {
    // Printing some messages to the user!
    println!("Guessing Game!");
    println!("Please enter your guess");

    // Generate a secret number
    // thread_rng() is a random number generator
    // the gen_range() function creates a random number from a range
    // The range is defined with start and end number => 1..=100, the numbers are included 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Printing the secret number
    println!("The secret number is {secret_number}");

    // Using the "let" keyword to define a new variable
    // Variables are immutable by default, so we add the "mut" keyword 
    // Uses the String type provided by the standard library 
    // Using "::" indicates that "new" is a function to the string type 
    // Summarize: creates a new mutable variable and binds it to an empty instance of string 
    let mut guess = String::new();


    // Using the stdin() function for receiving user input. Returns an instance of the Stdin library 
    // Calling th read_line() function for reading a line of input 
    // Give &mut guess to tell it where to store the user input, in this case the guess variable 
    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
    // read_line() returns a Result (enumeration value), will be Ok or Err
    // the expect() function will be called if result returns Err and then it will crash the program and display the message 
    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line");

    // Using println with the {} braces to create a f string. Can also use it with empty braces and add the variable as a comma behind:
    //      - println!("You guessed: {}", guess)
    println!("You guessed: {guess}");
}
