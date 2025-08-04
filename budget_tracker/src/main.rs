use std::io;

#[derive(Debug)]
enum TransactionType {
    Income,
    Expense,
}

#[derive(Debug)]
struct Transaction {
    t_type: TransactionType,
    amount: f64,
    description: String,
}

fn main() {
    let mut transactions: Vec<Transaction> = Vec::new();

    loop {
        println!("\n📋 Budget Tracker");
        println!("1. Add Income");
        println!("2. Add Expense");
        println!("3. View Transactions");
        println!("4. Show Balance");
        println!("5. Exit");

        let choice = get_input("Choose an option:");

        match choice.trim() {
            "1" => {
                let amount = get_amount("Enter income amount:");
                let description = get_input("Enter description:");
                transactions.push(Transaction {
                    t_type: TransactionType::Income,
                    amount,
                    description,
                });
                println!("✅ Income added!");
            }
            "2" => {
                let amount = get_amount("Enter expense amount:");
                let description = get_input("Enter description:");
                transactions.push(Transaction {
                    t_type: TransactionType::Expense,
                    amount,
                    description,
                });
                println!("✅ Expense added!");
            }
            "3" => {
                println!("\n📄 Transactions:");
                for (i, t) in transactions.iter().enumerate() {
                    let kind = match t.t_type {
                        TransactionType::Income => "Income",
                        TransactionType::Expense => "Expense",
                    };
                    println!("{}. {}: ${:.2} - {}", i + 1, kind, t.amount, t.description);
                }
            }
            "4" => {
                let mut balance = 0.0;
                for t in &transactions {
                    match t.t_type {
                        TransactionType::Income => balance += t.amount,
                        TransactionType::Expense => balance -= t.amount,
                    }
                }
                println!("💰 Current Balance: ${:.2}", balance);
            }
            "5" => {
                println!("👋 Exiting. Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option, try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{prompt} ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_amount(prompt: &str) -> f64 {
    loop {
        let input = get_input(prompt);
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("❌ Please enter a valid number."),
        }
    }
}
