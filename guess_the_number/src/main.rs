use rand::Rng;
use std::io;

fn main() {
    let random_num: i32 = gen_random_num();

    println!("\nLet's play the number guessing game! 😋");

    println!("\nGuess a number between 1 and 100: ");

    loop {
        let mut rough_input = String::new();
        println!("Number: ");

        io::stdin()
            .read_line(&mut rough_input)
            .expect("Failed to read line!");

        let input = rough_input.trim();

        if input.is_empty() {
            println!("Please enter a number.");
            continue;
        }

        let parsed_input = match input.parse::<i32>() {
            Ok(number) => number,
            Err(_) => {
                println!("This is not a valid number!");
                println!("Try again!");
                continue;
            }
        };

        if parsed_input == -1 {
            continue;
        } else if check_if_correct(&parsed_input, &random_num) {
            break;
        }
    }
}

fn gen_random_num() -> i32 {
    let mut rng = rand::thread_rng();
    rng.random_range(1..=100)
}

fn check_if_correct(guess: &i32, correct_num: &i32) -> bool {
    if guess == correct_num {
        println!("Yaay! You guessed correctly! 🥳🥳🎇🎉😎");
        true
    } else if guess > correct_num {
        println!("Go lower!");
        false
    } else {
        println!("Go higher!");
        false
    }
}
