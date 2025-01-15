// Guess the secret number!

// Imports the io module from library for input/output.
use std::io;
// Adds comparing/ordering for the 2 comparable values
use std::cmp::Ordering;
// Defines Rng trait for picking numbers in scope, Rng = random number generator
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // secret_number gives Rng specific to current thread, generates # within specified range 1 to 100. 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Declare a mutable variable `guess` to store the user's input. 
        // We initialize it as an empty string.
        let mut guess = String::new();

        // Read a line of input from the user and store it in the `guess` variable.
        // The 'read_line' method takes a mutable reference to `guess` as an argument.
        // If the input fails, the program will panic with the message "Failed to read line".
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Ensures a numerical value is inputted, not a random key
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Output the value of `guess` to the console.
        // The `println!` macro will print the value of the `guess` variable.
        // Rust's string interpolation syntax `{guess}` embeds the value of `guess`.    
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Break lets user quit after winning
                break;
            }
        }
    }   
}
