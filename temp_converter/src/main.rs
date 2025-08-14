use std::io;

fn main() {
    println!("🌡️ Temperature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("3: Celsius to Kelvin");
    println!("4: Kelvin to Celsius");

    println!("Enter your choice: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid choice!");
            return;
        }
    };

    println!("Enter the temperature value: ");
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Failed to read line");
    let temp: f64 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid temperature!");
            return;
        }
    };

    match choice {
        1 => println!("{:.2}°C = {:.2}°F", temp, celsius_to_fahrenheit(temp)),
        2 => println!("{:.2}°F = {:.2}°C", temp, fahrenheit_to_celsius(temp)),
        3 => println!("{:.2}°C = {:.2}K", temp, celsius_to_kelvin(temp)),
        4 => println!("{:.2}K = {:.2}°C", temp, kelvin_to_celsius(temp)),
        _ => println!("❌ Invalid choice!"),
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
}

fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}
