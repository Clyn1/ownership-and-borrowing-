use std::io; // For user input

fn main() {
    println!("=== Simple Calculator ===");

    // Get first number
    let num1 = read_number("Enter the first number: ");

    // Get second number
    let num2 = read_number("Enter the second number: ");

    // Get the operation
    println!("Choose an operation (+, -, *, /): ");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim(); // Remove extra spaces/newlines

    // Perform calculation
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed!");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    // Show result
    println!("Result: {}", result);
}

// Function to read and convert number from user input
fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid number! Please try again.");
            }
        }
    }
}
