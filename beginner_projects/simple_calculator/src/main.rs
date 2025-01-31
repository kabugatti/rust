use std::io;

struct Calculator {
    num1: f32,
    num2: f32,
}

impl Calculator {
    fn new(num1: f32, num2: f32) -> Calculator {
        // Fixed constructor
        Calculator {
            num1: if num1 != 0.0 { num1 } else { 0.0 },
            num2: if num2 != 0.0 { num2 } else { 0.0 },
        }
    }

    fn add(&self) -> f32 {
        self.num1 + self.num2
    }

    fn substract(&self) -> f32 {
        self.num1 - self.num2
    }

    fn multiply(&self) -> f32 {
        self.num1 * self.num2
    }

    fn divide(&self) -> f32 {
        self.num1 / self.num2
    }
}

fn main() {
    println!("Welcome to Grand's simple calculator!");

    loop {
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("0. Exit");

        let mut rough_input = String::new();

        io::stdin()
            .read_line(&mut rough_input)
            .expect("Failed to read line!");

        let parsed_input = rough_input.trim();

        if parsed_input.is_empty() {
            println!("Please enter a choice among the options!");
            continue;
        }

        let choice: i32 = match parsed_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => {
                let two_nums = get_two_nums();

                if two_nums.is_empty() {
                    continue;
                } else {
                    let calc = Calculator::new(two_nums[0], two_nums[1]);
                    let ans = calc.add();
                    println!(
                        "The sum of {} and {} is {:.2}",
                        two_nums[0], two_nums[1], ans
                    );
                    continue;
                }
            }
            2 => {
                let two_nums = get_two_nums();

                if two_nums.is_empty() {
                    continue;
                } else {
                    let calc = Calculator::new(two_nums[0], two_nums[1]);
                    let ans = calc.substract();
                    println!(
                        "The difference of {} and {} is {:.2}",
                        two_nums[0], two_nums[1], ans
                    );
                    continue;
                }
            }
            3 => {
                let two_nums = get_two_nums();

                if two_nums.is_empty() {
                    continue;
                } else {
                    let calc = Calculator::new(two_nums[0], two_nums[1]);
                    let ans = calc.multiply();
                    println!(
                        "The multiplication of {} and {} is {:.2}",
                        two_nums[0], two_nums[1], ans
                    );
                    continue;
                }
            }
            4 => {
                let two_nums = get_two_nums();

                if two_nums.is_empty() {
                    continue;
                } else {
                    let calc = Calculator::new(two_nums[0], two_nums[1]);
                    let ans = calc.divide();
                    println!(
                        "The division of {} and {} is {:.2}",
                        two_nums[0], two_nums[1], ans
                    );
                    continue;
                }
            }
            0 => {
                println!("Have a lovely day 😎");
                break;
            }
            _ => {
                println!("Try again!");
                continue;
            }
        }
    }
}

fn get_two_nums() -> Vec<f32> {
    let mut two_nums: Vec<f32> = Vec::new();

    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter num1: ");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line!");

    println!("Enter num2: ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line!");

    let num1: f32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number!");
            return vec![];
        }
    };

    let num2: f32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number!");
            return vec![];
        }
    };

    two_nums.push(num1);
    two_nums.push(num2);

    two_nums
}
