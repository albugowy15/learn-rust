use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default. To make a
        // variable mutable, we use the mut before the variable name.
        let mut guess = String::new();

        // read_line to take whatever the user types into standard input 
        // and append that into a string (without overwriting its contents).
        // The string argument needs to be mutable so the method can 
        // change the string’s content.
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // expect is a method that takes a message as a parameter and will crash the program and display the message if the Result is an Err value.

        // Shadow the previous value of guess with a new one.
        // Shadowing lets us reuse the guess variable name rather than 
        // forcing us to create two unique variables, such as guess_str 
        // and guess for example.
        // The trim method on a String instance will eliminate any 
        // whitespace at the beginning and end.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    

   
}
