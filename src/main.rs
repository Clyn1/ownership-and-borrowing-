fn main() {
    // Ownership
    let name = String::from("Clyn");
    print_name(name); // ownership is moved here

    // println!("{}", name); // ❌ This would cause an error because ownership moved

    // Borrowing (Immutable)
    let city = String::from("Kisumu");
    print_city(&city); // borrow city instead of moving it
    println!("City after borrowing: {}", city); // ✅ works fine

    // Mutable Borrowing
    let mut age = 25;
    change_age(&mut age); // borrow mutably
    println!("Updated age: {}", age); // ✅ updated value
}

// Takes ownership of the string
fn print_name(name: String) {
    println!("Name: {}", name);
}

// Borrows the string (does not take ownership)
fn print_city(city: &String) {
    println!("City: {}", city);
}

// Borrows the integer mutably to change it
fn change_age(age: &mut i32) {
    *age += 1;
}
