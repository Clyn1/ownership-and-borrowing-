use std::io;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n--- Contact Book ---");
        println!("1. Add Contact");
        println!("2. Show All Contacts");
        println!("3. Exit");

        let choice = get_input("Enter choice:");

        match choice.trim() {
            "1" => {
                let name = get_input("Enter name:");
                let phone = get_input("Enter phone:");
                let contact = Contact { name, phone };
                contacts.push(contact); // ownership of name & phone moved into contact
                println!("✅ Contact added.");
            }
            "2" => {
                show_contacts(&contacts); // borrowed read-only
            }
            "3" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => {
                println!("❌ Invalid option.");
            }
        }
    }
}

// Function that borrows the contact list immutably
fn show_contacts(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        println!("📭 No contacts found.");
    } else {
        println!("📇 Your Contacts:");
        for (i, contact) in contacts.iter().enumerate() {
            println!("{}. {} - {}", i + 1, contact.name, contact.phone);
        }
    }
}

// Helper function to get user input
fn get_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
