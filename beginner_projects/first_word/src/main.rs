use std::io;

fn main() {
    println!("Please enter a sentence, and I'll return the first word!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim();
    
    let mut index = 0;

    for (i, c) in input.char_indices() {
        if c == ' ' {
            index = i;
            break;
        }
    }

    let answer = &input[0..index];

    println!("{answer}");
}
