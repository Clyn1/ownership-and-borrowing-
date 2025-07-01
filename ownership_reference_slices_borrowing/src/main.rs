use std::io;

fn main() {
    println!("Enter a sentence:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let trimmed = input.trim();

    if trimmed.is_empty() {
        println!("You entered an empty sentence.");
        return;
    }

    let longest = find_longest_word(trimmed);
    let sorted = sort_words_by_length(trimmed);

    println!("\n✅ Longest word: '{}'", longest.unwrap_or("None"));

    println!("\n📊 Words sorted by length:");
    for word in sorted {
        println!("- {} ({} chars)", word, word.len());
    }
}

fn find_longest_word(text: &str) -> Option<&str> {
    let words = text.split_whitespace();
    let mut longest: Option<&str> = None;

    for word in words {
        match longest {
            Some(current) if word.len() > current.len() => longest = Some(word),
            None => longest = Some(word),
            _ => {}
        }
    }

    longest
}

fn sort_words_by_length(text: &str) -> Vec<&str> {
    let mut words: Vec<&str> = text.split_whitespace().collect();
    words.sort_by(|a, b| b.len().cmp(&a.len()));
    words
}
