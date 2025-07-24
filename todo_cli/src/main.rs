use std::io; // For user input

fn main() {
    let mut tasks: Vec<String> = Vec::new(); // 🧠 Vector to hold tasks

    loop {
        println!("\n--- To-Do List ---");
        println!("1. Add a task");
        println!("2. View tasks");
        println!("3. Exit");

        let choice = get_input("Enter your choice:");

        match choice.trim() {
            "1" => {
                let task = get_input("Enter your task:");
                tasks.push(task);
                println!("✅ Task added.");
            }
            "2" => {
                println!("\n📝 Your Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, task);
                }
            }
            "3" => {
                println!("👋 Goodbye, Clyn!");
                break;
            }
            _ => {
                println!("❌ Invalid choice. Try again.");
            }
        }
    }
}

// 📥 Function to get input from the user
fn get_input(prompt: &str) -> String {
    use std::io::Write;

    print!("{} ", prompt);
    io::stdout().flush().unwrap(); // Flush to print without waiting

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string() // Remove newlines and return
}
