use std::io; // obtain io rust library

fn main() { // create main function (program entry point)
    println!("Guess the number!"); // outputs "Guess the number!"
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