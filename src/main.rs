use std::io; // obtain io rust module
use rand::Rng; // obtain Rng rust module

fn main() { // create main function (program entry point)
    println!("Guess the number!"); // outputs "Guess the number!"
    
    let secret_number = rand::thread_rng().gen_range(1..100); // declare variable "secret_number", and assign it a random value between 1, inclusive, and 100, exclusive.
                                                  // this is a range expression (ex. start..end) where the lower bound is inclusive, however the upper bound is exclusive (ex. 1..100) will only return a number between 1 & 99
                                                  // ...to make both inclusive, change the syntax to "..=" (ex. 1..=100) will return a number between 1 & 100, inclusive
    println!("Please input your guess."); // outputs "Please input your guess."

    let mut guess = String::new(); // declare variable "guess", and make it mutable (adaptable) of type String
                 // unlike Java, a double colon (::) acts as a path separator, not a lambda. (Ex. "String" is the crate, while "new()" is a module/function)
    io::stdin() // instantiate a handle to input in a terminal (if "std::io" had not been imported, this could be called as "std::io::stdin")
        .read_line(&mut guess) // read the input, and store the value in the mutable variable "guess"
        .expect("Failed to read line"); // if program errors, "expect" will crash the program with the error "Failed to read line"

    println!("You guessed: {}", guess); // outputs "You guessed: ", followed by the value of "guess"
                        // similar to Python, {} serves as a placeholder and takes on the value of the positional parameter (ex. first {} in a string receives the value of the first value passed after the string)
}

// If the program is run and 4 is inputted by the user, the program should return:
// "You guessed: 4"