// vector_examples.rs
// A simple Rust program demonstrating Vec<T> usage for your GitHub

fn main() {
    // 1. Creating Vec<T>
    let mut chest: Vec<i32> = Vec::new();
    let mut chest2 = vec![10, 20, 30];

    // 2. .push(), .pop(), .len()
    chest.push(100);
    chest.push(200);
    println!("After pushes: {:?}", chest);

    if let Some(last) = chest.pop() {
        println!("Popped value: {}", last);
    }
    println!("Length now: {}", chest.len());

    // 3. Iteration
    // a) Consuming iteration (chest2 is moved)
    for value in chest2 {
        println!("Consumed item: {}", value);
    }
    // chest2 cannot be used hereafter

    // b) Borrowed iteration
    let mut numbers = vec![4, 5, 6];
    for &n in &numbers {
        println!("Peeked: {}", n);
    }

    // c) Mutable borrow iteration
    for n in &mut numbers {
        *n *= 2;
    }
    println!("Doubled in-place: {:?}", numbers);

    // 4. Slices
    let array = [100, 200, 300, 400];
    let window = &array[1..3];
    println!("Slice window: {:?}", window);

    // 5. Generics Example
    fn largest<T: PartialOrd + Copy>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }
    let x = largest(3, 7);
    let y = largest(3.2, 4.8);
    println!("Largest ints: {}, largest floats: {}", x, y);

    // 6. Mini-project summary
    // Combine creation, push, pop, iteration, slice and generics
    let mut names: Vec<String> = Vec::new();
    names.push("Alice".to_owned());
    names.push("Bob".to_owned());
    names.push("Carol".to_owned());
    println!("Names: {:?}", names);

    let removed = names.pop().expect("Should have a name");
    println!("Removed: {}", removed);
    println!("Remaining count: {}", names.len());

    for name in &mut names {
        *name = name.to_uppercase();
    }
    println!("Uppercased names: {:?}", names);

    let slice: &[String] = &names[..];
    println!("Name slice view: {:?}", slice);
}
