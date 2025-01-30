use std::io;

fn main() {
    println!("Hello, let's convert temperature from celcius to fahrenheit and vice versa");

    loop {
        println!("1. Celcius to Fahrenheit");
        println!("2. Fahrenheit to Celcius");
        println!("3. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");

        let parsed_choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value, please enter a valid number!");
                continue;
            }
        };

        match parsed_choice {
            1 => {
                println!("Enter the value to be converted to fahrenheit: ");

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line!");

                let mut parsed_input: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a valid number!");
                        continue;
                    }
                };

                celcius_to_fahrenheit(&mut parsed_input);
                continue;
            }
            2 => {
                println!("Enter the value to be converted to Celcius");

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line!");

                let mut parsed_input: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a valid number!");
                        continue;
                    }
                };

                fahr_to_celcius(&mut parsed_input);
                continue;
            }
            3 => {
                println!("Have a lovely day! 😎");
                break;
            }
            _ => {
                println!("Invalid choice. Try again.");
                continue;
            }
        }
    }
}

fn fahr_to_celcius(fahr: &mut i32) -> f32 {
    let celcius = (*fahr as f32 - 32.0) * (5.0 / 9.0) as f32;

    println!("{} to Celcius is {:.2}", fahr, celcius);

    return celcius;
}

fn celcius_to_fahrenheit(celcius: &mut i32) -> f32 {
    let fahrenheit = (*celcius as f32 * (9.0 / 5.0)) + 32.0 as f32;

    println!("{} to Fahrenheit is {:.2}", celcius, fahrenheit);
    return fahrenheit;
}
