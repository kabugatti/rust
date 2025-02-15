use std::collections::HashMap;
use std::io;

fn main() {
    let mut words = HashMap::<String, u8>::new();

    loop {
        let mut input = String::new();

        println!("1. Play");
        println!("0. Exit");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let choice: i8 = input.trim().parse().unwrap_or(-1);

        match choice {
            1 => {
                count_frequency(&mut words);
            }
            _ => {
                println!("Have a nice day! 😎");
                break;
            }
        }
    }
}

fn count_frequency(words: &mut HashMap<String, u8>) {
    let mut input = String::new();

    println!("Enter some words");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let filtered_words: Vec<String> = input
        .trim()
        .to_lowercase()
        .split_whitespace()
        .map(String::from)
        .collect();

        for word in filtered_words {
            *words.entry(word).or_insert(0) += 1;
        }
}
