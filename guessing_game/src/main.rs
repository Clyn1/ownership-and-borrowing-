use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("🎲 Welcome to the Number Guessing Game!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("I have picked a number between 1 and 100. Can you guess it?");

        let mut attempts = 0;

        loop {
            println!("👉 Enter your guess:");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read input");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("⚠️ Please enter a valid number!");
                    continue;
                }
            };

            attempts += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("🔽 Too small! Try again."),
                Ordering::Greater => println!("🔼 Too big! Try again."),
                Ordering::Equal => {
                    println!("🎉 You got it in {attempts} attempts!");
                    break;
                }
            }
        }

        println!("Do you want to play again? (y/n)");

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read input");

        if play_again.trim().to_lowercase() != "y" {
            println!("👋 Thanks for playing! Goodbye.");
            break;
        }
    }
}
