use std::io;

fn main() {
    loop {
        println!("Enter a number from 1 to 100: ");
        println!("(Enter 0 to exit)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let cleaned: i32 = match input.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Have a great day!");
                    break;
                } else if num > 100 {
                    println!("Enter a number between 1 to 100");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        };

        for i in 1..=cleaned {
            if i % 3 == 0 && i % 5 == 0 {
                println!("FizzBuzz");
            } else if i % 3 == 0 {
                println!("Fizz");
            } else if i % 5 == 0 {
                println!("Buzz");
            } else {
                println!("{}", i);
            }
        }
    }
}
