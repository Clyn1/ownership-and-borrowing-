use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use chrono::Local; // For date and time

fn main() {
    loop {
        println!("\n📔 Personal Diary");
        println!("1. Add Entry");
        println!("2. View Entries");
        println!("3. Exit");

        let choice = read_input("Choose an option: ");

        match choice.trim() {
            "1" => add_entry(),
            "2" => view_entries(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_entry() {
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let entry = read_input("Write your diary entry: ");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("diary.txt")
        .expect("Failed to open diary file");

    writeln!(file, "{} - {}", date, entry).expect("Failed to write entry");

    println!("✅ Entry added!");
}

fn view_entries() {
    match fs::read_to_string("diary.txt") {
        Ok(contents) => {
            if contents.trim().is_empty() {
                println!("No entries yet.");
            } else {
                println!("\n--- Diary Entries ---\n{}", contents);
            }
        }
        Err(_) => println!("No diary found."),
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
