// fn main() {
//     println!("Hello, world!");
// }

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nGuess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=51);

    let mut guesses: u32 = 0;

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("\nYou win!\n");
                break;
            }
        }
    }
    println!("Você usou {guesses} tentativas!");
}
