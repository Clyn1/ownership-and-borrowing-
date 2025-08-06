use rand::Rng;
use std::io;

fn main() {
    println!("🎯 Welcome to the Guessing Game!");
    println!("You have 5 attempts to guess the number between 1 and 100.");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut attempts = 5;

        while attempts > 0 {
            println!("\nAttempts left: {}", attempts);
            println!("Enter your guess:");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("⚠️ Please enter a valid number.");
                    continue;
                }
            };

            if guess == secret_number {
                println!("🎉 Correct! You guessed the number.");
                break;
            } else if guess < secret_number {
                println!("📉 Too low!");
            } else {
                println!("📈 Too high!");
            }

            attempts -= 1;
        }

        if attempts == 0 {
            println!("💥 Out of attempts! The number was: {}", secret_number);
        }

        println!("\nDo you want to play again? (y/n):");

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            println!("👋 Thanks for playing! Goodbye!");
            break;
        }
    }
}
