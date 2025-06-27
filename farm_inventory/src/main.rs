fn main() {
    let mut inventory = Vec::new(); // Heap-allocated dynamic list

    add_item(&mut inventory, String::from("Maize"));
    add_item(&mut inventory, String::from("Beans"));
    add_item(&mut inventory, String::from("Sorghum"));

    display_inventory(&inventory);
    let total = count_items(&inventory);
    println!("Total items in inventory: {total}");
}

// Mutable borrow to add a new product
fn add_item(inventory: &mut Vec<String>, item: String) {
    inventory.push(item); // ownership of item is moved here
}

// Immutable borrow to display inventory
fn display_inventory(inventory: &Vec<String>) {
    println!("\n📦 Farm Inventory:");
    for item in inventory {
        println!("- {item}");
    }
}

// Immutable borrow to count items
fn count_items(inventory: &Vec<String>) -> usize {
    inventory.len()
}
