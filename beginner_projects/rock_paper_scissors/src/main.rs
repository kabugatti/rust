use rand::Rng;
use std::io;

// 0, 1, 2
// P, S, R

fn main() {
    let options = [
        String::from("Rock"),
        String::from("Paper"),
        String::from("Scissors"),
    ];
    loop {
        println!("1. Play Rock, Paper, Scissors!");
        println!("0. Exit");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        match user_input.trim().parse::<i32>() {
            Ok(num) => {
                if num == 0 {
                    println!("Have a lovely day! 😎");
                    break;
                }
            }
            Err(_) => {
                println!("Please choose among the options!");
                continue;
            }
        }

        // the game
        println!("1. Rock");
        println!("2. Paper");
        println!("3. Scissors");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        let user_choice: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Choose a number among the options. i.e. 1, 2 or 3");
                continue;
            }
        };

        let system_choice: i32 = gen_random_num();

        let mut winner: i32 = 0;

        if user_choice == system_choice {
            println!(
                "Your choice: {}, My choice: {}",
                options[(user_choice - 1) as usize],
                options[(system_choice - 1) as usize]
            );
            println!("Draw!");
            continue;
        } else if user_choice > system_choice {
            winner = check_winner(user_choice, system_choice);
            println!(
                "Your choice: {}, My choice: {}",
                options[(user_choice - 1) as usize],
                options[(system_choice - 1) as usize]
            );
            if winner == user_choice {
                println!("You won!");
                continue;
            }
        }

        println!(
            "Your choice: {}, My choice: {}",
            options[(user_choice - 1) as usize],
            options[(system_choice - 1) as usize]
        );
        println!("I won!");
        continue;
    }
}

fn check_winner(num1: i32, num2: i32) -> i32 {
    let diff = (num1 - num2) % 4;

    if diff == 1 {
        return num1;
    }

    num2
}

fn gen_random_num() -> i32 {
    let mut rng = rand::thread_rng();
    rng.random_range(1..=3)
}
