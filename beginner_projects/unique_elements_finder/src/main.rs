use std::collections::BTreeSet;

fn main() {
    let numbers = vec![4, 2, 8, 4, 2, 9, 8];

    let mut unique_numbers = BTreeSet::<u8>::new();

    for num in numbers.into_iter() {
        unique_numbers.insert(num);
    }

    println!("Unique numbers: {:?}", unique_numbers);
}
