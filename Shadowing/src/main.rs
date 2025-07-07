use std::io;

fn main() {
    println!("Enter a sentence:");

    // Step 1: Get user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Step 2: Shadow to trim whitespace
    let input = input.trim();

    // Step 3: Shadow to lowercase
    let input = input.to_lowercase();

    // Step 4: Shadow to get word count
    let input = input.split_whitespace().count();

    println!("Word count: {}", input);
}
