use std::io;

fn main() {
    loop {
        println!("1. Do fibonacci sequence");
        println!("0. Exit");

        let mut rough_input = String::new();

        io::stdin()
            .read_line(&mut rough_input)
            .expect("Failed to read line!");

        let number: i32 = match rough_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        if number == 0 {
            println!("Have a lovely day! 😎");
            break;
        }

        println!("Enter number of sequence: ");

        let mut seq = String::new();

        io::stdin()
            .read_line(&mut seq)
            .expect("Failed to read line!");

        let seq: i32 = match seq.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let result = fibonacci_sequence(&seq);
        println!(
            "The first {} numbers in the fibonacci sequence are {:?}",
            seq, result
        );
    }
}

fn fibonacci_sequence(seq: &i32) -> Vec<i32> {
    let mut fib_seq: Vec<i32> = vec![0, 1];

    let seq_length = *seq - fib_seq.len() as i32;

    for _i in 0..seq_length {
        let last_digit = fib_seq[fib_seq.len() - 1];
        let second_last_digit = fib_seq[fib_seq.len() - 2];

        let new_digit = last_digit + second_last_digit;

        fib_seq.push(new_digit);
    }

    fib_seq
}
