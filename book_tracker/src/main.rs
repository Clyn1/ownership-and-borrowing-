use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    read: bool,
}

fn load_books() -> Vec<Book> {
    let data = fs::read_to_string("books.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_books(books: &Vec<Book>) {
    let data = serde_json::to_string_pretty(books).expect("Failed to serialize books");
    fs::write("books.json", data).expect("Unable to write file");
}

fn main() {
    let mut books = load_books();

    loop {
        println!("\n📚 Book Tracker");
        println!("1. Add a book");
        println!("2. List all books");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut title = String::new();
                let mut author = String::new();

                print!("Enter book title: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).unwrap();

                print!("Enter author name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut author).unwrap();

                let book = Book {
                    title: title.trim().to_string(),
                    author: author.trim().to_string(),
                    read: false,
                };
                books.push(book);
                save_books(&books);
                println!("✅ Book added!");
            }
            "2" => {
                println!("\n📖 Your Books:");
                for (i, book) in books.iter().enumerate() {
                    println!(
                        "{}. {} by {} [{}]",
                        i + 1,
                        book.title,
                        book.author,
                        if book.read { "Read" } else { "Unread" }
                    );
                }
            }
            "3" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("⚠️ Invalid choice, try again."),
        }
    }
}
