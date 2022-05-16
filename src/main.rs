extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim() to eliminate the spaces from begin and from end
        // parse() para converter
        // The type u8 means 8 bits which can contain up to 255 numbers (2**8)
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Very low!"),
            Ordering::Greater => println!("Very high!"),
            Ordering::Equal => {
                println!("You're right!");
                break;
            }
        }
    }
}
