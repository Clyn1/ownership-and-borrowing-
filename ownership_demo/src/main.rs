use std::io::{self, Write}; // Import I/O and Write for flushing output

fn main() {
    // Create a mutable vector of Strings to store book titles; we own this Vec
    let mut books: Vec<String> = Vec::new();

    loop {
        // Display available commands
        println!("\nCommands: add, list, remove, quit");

        // Print prompt without newline and flush so it appears immediately
        print!("> ");
        io::stdout().flush().unwrap();

        // Read user input into a new String
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim whitespace/newline and store as &str
        let cmd = input.trim();

        // Match the command and call the appropriate function
        match cmd {
            "add" => add_book(&mut books),       // Mutable borrow: allows adding a book
            "list" => list_books(&books),        // Immutable borrow: read-only listing
            "remove" => remove_book(&mut books), // Mutable borrow: allows removal
            "quit" => break,                     // Exit the loop
            _ => println!("Unknown command"),    // Handle any other input
        }
    }

    // Program end message
    println!("Goodbye!");
}

// Function to list books (immutable borrow)
fn list_books(books: &Vec<String>) {
    // Check if vector is empty
    if books.is_empty() {
        println!("(no books)");
        return;
    }
    // Iterate with index and title
    for (i, title) in books.iter().enumerate() {
        println!("{}: {}", i, title);
    }
}

// Function to add a book (mutable borrow)
fn add_book(books: &mut Vec<String>) {
    println!("Enter book title:");
    let mut title = String::new();            // Create new String for input
    io::stdin().read_line(&mut title).unwrap();

    // Trim newline and convert to owned String
    let title = title.trim().to_string();

    // Move the title into the vector
    books.push(title);
    println!("Book added!");
}

// Function to remove a book by index (mutable borrow)
fn remove_book(books: &mut Vec<String>) {
    // Show current list first
    list_books(books);
    println!("Enter index to remove:");

    let mut idx_str = String::new();          // String to hold index input
    io::stdin().read_line(&mut idx_str).unwrap();

    // Try to parse input into usize
    if let Ok(idx) = idx_str.trim().parse::<usize>() {
        if idx < books.len() {
            // Remove returns the String, transferring ownership to 'removed'
            let removed = books.remove(idx);
            println!("Removed \"{}\"", removed);
            return;
        }
    }
    println!("Invalid index");
}
