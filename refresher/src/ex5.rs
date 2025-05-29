// Work with HashMaps and more complex vector operations
use std::collections::HashMap;

fn main() {
    // Create a word frequency counter
    let text = "the quick brown fox jumps over the lazy dog the fox is quick";
    
    // Your task: create a HashMap that counts how many times each word appears
    let mut word_count = HashMap::<String, i32>::new();
    
    // Split the text into words and count them
    // Hint: use text.split_whitespace() and HashMap's entry() method

    text.split_whitespace().for_each(|word| {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    });
    
    // Print the results
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}
