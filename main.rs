fn main() {
    // Ownership
    let name = String::from("Clinton");
    greet_owner(name); // Ownership moved into the function

    // let name2 = name; // ❌ This would cause an error if uncommented

    // Borrowing with immutable reference
    let city = String::from("Kisumu");
    print_location(&city); // We're borrowing city, not taking ownership

    // Borrowing with mutable reference
    let mut status = String::from("beginner");
    update_status(&mut status);
    println!("Updated status: {}", status);

    // Lifetimes example
    let word1 = String::from("Rust");
    let word2 = String::from("Ownership");
    let result = longest(&word1, &word2);
    println!("Longest word is: {}", result);
}

// Ownership: function takes ownership of the String
fn greet_owner(owner: String) {
    println!("Hello, {}! You own this data now.", owner);
}

// Immutable borrow: no ownership taken
fn print_location(place: &String) {
    println!("You are coding from: {}", place);
}

// Mutable borrow: allows changing the data
fn update_status(level: &mut String) {
    level.push_str(" Rustacean");
}

// Lifetimes: ensures references live long enough
fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
