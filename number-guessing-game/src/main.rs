/*
 * Number Guessing Game Build a program that generates a random 
 * number between 1 and 100 using the rand crate. 
 * Prompt the user to guess the number in a loop. 
 * Use a match expression to compare the guess to the secret number, 
 * printing "Too high!", "Too low!", or "You win!" and breaking the 
 * loop when they guess correctly.
*/

use std::io;

struct UserGuess {
    number: i32
}

impl UserGuess {
    fn new(guess: String) -> Result<UserGuess, String> {
        let parsed: Result<i32, _> = guess.trim().parse();

        match parsed {
            Ok(number) => match number {
                0..=100 => Ok(UserGuess { number }),
                _ => Err(format!("Number {number} is not within range"))
            },
            Err(e) => Err(format!("{e}"))
        }
    }
    
}

fn main() {
    let secret_number = rand::random_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Enter your guess: ");
        if let Err(e) = io::stdin().read_line(&mut guess) {
            println!("Failed reading line {e}");
            continue;
        }
        let guess = UserGuess::new(guess);
        match guess {
            Ok(number) => match number {
                n if n.number < secret_number => println!("Your guess ({}) is too low", n.number),
                n if n.number > secret_number => println!("Your guess ({}) is too high", n.number),
                _ => {
                    println!("Yes! That's the number");
                    break;
                }
            },
            Err(msg) => println!("Something went wrong: {msg}")
        }
    }
}