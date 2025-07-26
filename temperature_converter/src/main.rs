use std::io;

fn main() {
    println!("🌡️ Temperature Converter (Celsius ↔ Fahrenheit)");
    println!("Type 'exit' anytime to quit.\n");

    loop {
        let direction = get_input("Convert to (C)elsius or (F)ahrenheit?");

        if direction.to_lowercase() == "exit" {
            break;
        }

        let temp_input = get_input("Enter the temperature:");
        if temp_input.to_lowercase() == "exit" {
            break;
        }

        let temp_value: f64 = match temp_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("❌ Please enter a valid number.\n");
                continue;
            }
        };

        match direction.trim().to_lowercase().as_str() {
            "c" => {
                let celsius = fahrenheit_to_celsius(temp_value);
                println!("✅ {temp_value}°F = {celsius:.2}°C\n");
            }
            "f" => {
                let fahrenheit = celsius_to_fahrenheit(temp_value);
                println!("✅ {temp_value}°C = {fahrenheit:.2}°F\n");
            }
            _ => {
                println!("❌ Invalid direction. Enter 'C' or 'F'.\n");
            }
        }
    }

    println!("👋 Goodbye!");
}

fn get_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{prompt} ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
