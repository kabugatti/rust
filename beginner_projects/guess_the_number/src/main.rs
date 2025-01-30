use rand::Rng;
use std::io;

fn main() {
    // Get the random number
    let random_number = gen_random_num();

    println!("\nLet's play 'Guess the number'! 🤗");

    println!("Guess a number between 1 and 100.");

    
    loop {
        let mut rough_input = String::new();

        io::stdin()
            .read_line(&mut rough_input)
            .expect("Failed to read line!");

        let input = rough_input.trim();

        if input.is_empty() {
            println!("Please enter a number!");
            continue;
        }

        let parsed_input = match input.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number.");
                continue;
            }
        };

        let is_correct = check_if_correct(&parsed_input, &random_number);

        if is_correct {
            break;
        }
    }
}

fn gen_random_num() -> i32 {
    let mut rng = rand::thread_rng();
    rng.random_range(1..=100)
}

fn check_if_correct(guess: &i32, correct: &i32) -> bool {
    if guess == correct {
        println!("Yaay! You correctly guessed {}! 🎉🥳🎇😎💯✅", correct);
        return true;
    } else if guess > correct {
        println!("Go lower!");
    } else {
        println!("Go higher!");
    }

    return false;
}
