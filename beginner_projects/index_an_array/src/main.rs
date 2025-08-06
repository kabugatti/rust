use std::io;

fn main() {
    let numbers = [2, 9, 1, 4, 6];

    println!("Please enter an index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Failed to enter invalid number");

    let indexed_number = numbers[index];

    println!("The number at index {index} is {indexed_number}");
}
