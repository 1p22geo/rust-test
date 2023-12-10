use std::{cmp::Ordering, io};

use rand::Rng;

use crate::guess::Guess;

mod guess;

#[cfg(test)]
mod tests;

fn main() {
    println!("Guess the number!");
    let number: Guess = Guess::new(rand::thread_rng().gen_range(1..=100));

    loop {
        println!("Please type in your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read stdin");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            _ => continue,
        };
        if !Guess::check(guess) {
            println!("Number must be between 1 and 100");
            continue;
        }
        let guess = Guess::new(guess);

        match guess.cmp(&number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("{} is too large!", guess.value());
                continue;
            }
            Ordering::Less => {
                println!("{} is too small", guess.value());
                continue;
            }
        }
    }
}
