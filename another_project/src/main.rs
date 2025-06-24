fn main() {
    println!("🚀 Welcome to Rust Ownership Demo!");

    // 💼 Step 1: Ownership
    let name = String::from("Clyn");
    greet_owner(name); // Ownership is moved here

    // println!("Hello again, {}", name); // ❌ This line would cause an error because 'name' was moved

    // ✅ Step 2: Copy types
    let age = 24;
    show_age(age); // 'age' is copied, so we can still use it
    println!("Your age is still: {}", age);

    // 🔁 Step 3: Ownership returned
    let food = String::from("Mangoes");
    let returned_food = give_back(food);
    println!("You got back your food: {}", returned_food);
}

fn greet_owner(n: String) {
    println!("Hello, {}! You now own this String.", n);
}

fn show_age(a: i32) {
    println!("You are {} years old.", a);
}

fn give_back(f: String) -> String {
    println!("Eating {} and giving it back!", f);
    f
}
