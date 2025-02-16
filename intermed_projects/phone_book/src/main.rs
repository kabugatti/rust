// #![allow(dead_code)]

use std::collections::BTreeMap;
use std::io;

fn main() {
    let mut phone_book = BTreeMap::<String, String>::new();

    loop {
        println!("\n1. Add an entry");
        println!("2. Remove an entry");
        println!("3. Update an entry");
        println!("4. Search for number by name");
        println!("5. Get all entries");
        println!("0. Exit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let choice: i8 = input.trim().parse().unwrap_or(-1);

        match choice {
            0 => {
                println!("Have a great day! 😎");
                break;
            },
            1 => add_entry(&mut phone_book),
            2 => remove_entry(&mut phone_book),
            3 => update_entry(&mut phone_book),
            4 => search_entry(&mut phone_book),
            5 => get_entries(&mut phone_book),
            _ => {
                println!("Please select from the options given!");
                continue;
            }
        }
    }
}

fn add_entry(pb: &mut BTreeMap<String, String>) {
    let mut name = String::new();
    let mut number = String::new();

    println!("Enter name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line!");

    println!("Enter phone number: ");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    name = name.trim().to_lowercase();
    number = number.trim().to_lowercase();

    pb.insert(name.clone(), number.clone());

    println!("The following entry has been added: {}: {}", name, number);
}

fn remove_entry(pb: &mut BTreeMap<String, String>) {
    let mut name = String::new();

    println!("Enter name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line!");

    name = name.trim().to_lowercase();

    if let Some(_val) = pb.get(&name) {
        pb.remove(&name);
        println!("{} has been removed!", name.clone());
    } else {
        println!("Name does not exist in phone book!");
    }
}

fn update_entry(pb: &mut BTreeMap<String, String>) {
    let mut name = String::new();
    let mut number = String::new();

    println!("Enter name to update: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line!");

    name = name.trim().to_lowercase();

    if let Some(val) = pb.get(&name) {
        println!("Enter new phone number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");
        
        number = number.trim().to_lowercase();

        println!("{}'s number updated from {} to {}", name.clone(), val.clone(), number.clone());
        
        pb.insert(name, number);
    } else {
        println!("That name doesn't exist in the phone book!");
    }

    
}

fn search_entry(pb: &mut BTreeMap<String, String>) {
    let mut name = String::new();

    println!("Enter name of person: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line!");

    name = name.trim().to_lowercase();

    if let Some(val) = pb.get(&name) {
        println!("Here you go! {}: {}", name, val);
    } else {
        println!("Name does not exist in phone book!");
    }
}

fn get_entries(pb: &mut BTreeMap<String, String>) {
    // println!("Entries in phone book: {:?}", pb);
    let mut i = 1;
    println!("\nName: Phone Number");

    for (name, number) in pb.iter() {
        println!("{}.{}: {}", i, name, number);
        i += 1;
    }
}
