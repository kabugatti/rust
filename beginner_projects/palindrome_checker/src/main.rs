use std::io;

fn main() {
    println!("Enter a string: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let is_pal = is_palindrome(&input);

    if is_pal {
        println!("{} is a palindrome!", input.trim());
    } else {
        println!("Not a palindrome!");
    }
}

fn is_palindrome(s: &String) -> bool {
    // Clean the input string
    let cleaned = s
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    // Create reversed version
    let reversed: String = cleaned.chars().rev().collect();

    // Compare and return result
    cleaned == reversed
}
