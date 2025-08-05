fn main() {
    // Scalar types
    let temperature: f64 = 29.5;
    let humidity: u8 = 75;
    let is_raining: bool = true;
    let day_initial: char = 'T'; // T for Tuesday, for example

    // Tuple for city info
    let city_info: (&str, u32, i32) = ("Kisumu", 610_000, 1131);
    let (city_name, population, elevation) = city_info;

    // Array of 5 temperature readings
    let temp_readings: [f64; 5] = [28.3, 29.5, 30.0, 27.8, 26.9];

    // Print the weather details
    println!("Weather Report for: {}", city_name);
    println!("Population: {} | Elevation: {} meters", population, elevation);
    println!("Temperature now: {}°C", temperature);
    println!("Humidity: {}%", humidity);
    println!("Is it raining? {}", is_raining);
    println!("Day: {}", day_initial);

    // Print the temperature readings
    println!("\nReadings throughout the day: {:?}", temp_readings);

    // Bonus: Calculate average of first two readings
    let average_temp = (temp_readings[0] + temp_readings[1]) / 2.0;
    println!("Average of morning and afternoon: {:.1}°C", average_temp);
}
