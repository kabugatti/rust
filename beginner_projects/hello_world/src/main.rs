use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("\nHello there! What's your name?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        if input.trim().is_empty() {
            println!("Please enter your name.");
            continue;
        } else {
            break;
        }
    }

    println!("\nHi {}, nice to meet you! 😊", input.trim());
}
