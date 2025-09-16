use rand::Rng; // add rand in Cargo.toml

fn main() {
    println!("🎲 Dice Roller!");

    let mut input = String::new();
    println!("How many dice do you want to roll?");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_dice: u32 = input.trim().parse().expect("Please enter a number");

    input.clear();
    println!("How many sides per die?");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let sides: u32 = input.trim().parse().expect("Please enter a number");

    let mut rng = rand::thread_rng();
    let mut total = 0;

    println!("Rolling {} dice with {} sides each...", num_dice, sides);
    for i in 1..=num_dice {
        let roll = rng.gen_range(1..=sides);
        println!("Die {}: {}", i, roll);
        total += roll;
    }

    println!("Total: {}", total);
}
