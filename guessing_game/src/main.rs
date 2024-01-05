use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number! Try again.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Nice! You guessed my number.");
                break;
            }
            Ordering::Less => println!("Nice try, but that is not my number. Try a bigger number!"),
            Ordering::Greater => {
                println!("Nice try, but that is not my number. Try a smaller number!")
            }
        };
    }
}
