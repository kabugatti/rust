// RUST CORE CONCEPTS PRACTICE (LESSONS 1-8)
// Focused exercises on the fundamentals you've learned so far

use std::collections::HashMap;

// ============================================================================
// EXERCISE 1: Variables, Data Types, and Mutability
// Understanding: Rust variables are immutable by default, explicit typing when needed
// ============================================================================

fn exercise_1_variables_and_types() {
    println!("=== Exercise 1: Variables and Data Types ===");
    
    // Practice with different data types
    let integer: i32 = 42;
    let float = 3.14159; // Rust infers f64
    let boolean = true;
    let character = '🦀';
    
    // Tuples - grouping different types
    let person: (&str, i32, bool) = ("Alice", 30, true);
    println!("Person: name={}, age={}, is_active={}", person.0, person.1, person.2);
    
    // Destructuring tuples
    let (name, age, is_active) = person;
    println!("Destructured: {} is {} years old, active: {}", name, age, is_active);
    
    // Arrays - fixed size, same type
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 10]; // Array of 10 zeros
    
    println!("First day: {}", days[0]);
    println!("Array length: {}", numbers.len());
    println!("Zeros array: {:?}", zeros);
    
    // Mutability demonstration
    let mut counter = 0;
    counter += 1;
    counter *= 2;
    println!("Counter after operations: {}", counter);
    
    // Shadowing - redefining variables
    let x = 5;
    let x = x + 1; // Shadows the previous x
    let x = x * 2; // Shadows again
    println!("Final x value: {}", x);
    
    // Type conversion
    let integer_value = 65;
    let character_value = integer_value as u8 as char;
    println!("Integer {} as character: {}", integer_value, character_value);
}

// ============================================================================
// EXERCISE 2: Functions and Control Flow
// Understanding: Functions, expressions vs statements, control structures
// ============================================================================

// Function that returns a value (expression)
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // No semicolon = expression = return value
}

// Function with multiple return points
fn describe_number(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else if n < 10 {
        "single digit"
    } else {
        "multiple digits"
    }
}

// Function demonstrating different loop types
fn demonstrate_loops() {
    println!("\n--- Loop Demonstrations ---");
    
    // Basic loop with break
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2; // Return value from loop
        }
    };
    println!("Loop result: {}", result);
    
    // While loop
    let mut number = 3;
    while number != 0 {
        println!("Countdown: {}", number);
        number -= 1;
    }
    
    // For loop with range
    println!("Counting up:");
    for i in 1..=5 {
        println!("  {}", i);
    }
    
    // For loop with array
    let animals = ["cat", "dog", "bird"];
    for animal in animals.iter() {
        println!("Animal: {}", animal);
    }
    
    // For loop with enumeration
    for (index, animal) in animals.iter().enumerate() {
        println!("Animal #{}: {}", index + 1, animal);
    }
}

fn exercise_2_functions_and_control() {
    println!("\n=== Exercise 2: Functions and Control Flow ===");
    
    let sum = add_numbers(10, 20);
    println!("Sum: {}", sum);
    
    let test_numbers = [-5, 0, 7, 15];
    for num in test_numbers.iter() {
        println!("{} is {}", num, describe_number(*num));
    }
    
    demonstrate_loops();
    
    // Match expressions (pattern matching)
    let grade = 85;
    let letter_grade = match grade {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("Grade {} is letter grade: {}", grade, letter_grade);
    
    // Match with multiple values
    let day_number = 3;
    let day_type = match day_number {
        1 | 7 => "Weekend",
        2..=6 => "Weekday",
        _ => "Invalid day",
    };
    println!("Day {} is a {}", day_number, day_type);
}

// ============================================================================
// EXERCISE 3: Ownership Deep Dive
// Understanding: Move semantics, borrowing rules, references
// ============================================================================

// Function that takes ownership
fn take_ownership(s: String) {
    println!("I now own: {}", s);
} // s goes out of scope and is dropped

// Function that borrows
fn borrow_string(s: &String) {
    println!("I'm borrowing: {}", s);
} // s goes out of scope, but nothing is dropped

// Function that mutably borrows
fn modify_string(s: &mut String) {
    s.push_str(" - modified!");
}

// Function that returns ownership
fn give_ownership() -> String {
    String::from("This string is yours")
}

// Function that takes and returns ownership
fn take_and_return(s: String) -> String {
    println!("Processing: {}", s);
    s // Return ownership back
}

fn exercise_3_ownership() {
    println!("\n=== Exercise 3: Ownership Deep Dive ===");
    
    // Basic ownership transfer
    let s1 = String::from("hello");
    take_ownership(s1);
    // s1 can no longer be used here
    
    // Borrowing instead of moving
    let s2 = String::from("world");
    borrow_string(&s2);
    println!("I can still use s2: {}", s2);
    
    // Mutable borrowing
    let mut s3 = String::from("original");
    println!("Before: {}", s3);
    modify_string(&mut s3);
    println!("After: {}", s3);
    
    // Getting ownership from function
    let s4 = give_ownership();
    println!("Received: {}", s4);
    
    // Passing and receiving ownership
    let s5 = String::from("test");
    let s5_back = take_and_return(s5);
    println!("Got back: {}", s5_back);
    
    // Multiple references demonstration
    let s6 = String::from("multiple refs");
    let r1 = &s6;
    let r2 = &s6;
    println!("r1: {}, r2: {}", r1, r2);
    
    // Scope and lifetime demonstration
    {
        let s7 = String::from("scoped");
        println!("Inside scope: {}", s7);
    } // s7 is dropped here
    
    // Demonstrate the borrowing rules
    demonstrate_borrowing_rules();
}

fn demonstrate_borrowing_rules() {
    println!("\n--- Borrowing Rules Demo ---");
    
    let mut data = String::from("data");
    
    // Rule 1: Multiple immutable references are OK
    let ref1 = &data;
    let ref2 = &data;
    println!("Immutable refs: {} and {}", ref1, ref2);
    
    // Rule 2: Only one mutable reference at a time
    let mut_ref = &mut data;
    mut_ref.push_str(" updated");
    println!("Mutable ref: {}", mut_ref);
    
    // Rule 3: Cannot mix mutable and immutable references
    // This would not compile if uncommented:
    // let another_ref = &data; // Cannot borrow as immutable
    // println!("{}", mut_ref); // while borrowed as mutable
}

// ============================================================================
// EXERCISE 4: Structs and Method Syntax
// Understanding: Custom types, associated functions, methods
// ============================================================================

#[derive(Debug)] // Allows printing with {:?}
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    // Associated function (constructor)
    fn new(name: String, age: u32, email: String) -> Person {
        Person { name, age, email }
    }
    
    // Convenience constructor
    fn new_simple(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            email: String::from("no-email@example.com"),
        }
    }
    
    // Method that borrows self
    fn introduce(&self) -> String {
        format!("Hi, I'm {} and I'm {} years old", self.name, self.age)
    }
    
    // Method that checks a condition
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    // Method that mutably borrows self
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy birthday {}! Now {} years old.", self.name, self.age);
    }
    
    // Method that takes ownership (consumes the struct)
    fn into_summary(self) -> String {
        format!("{} (age {})", self.name, self.age)
    }
}

// Tuple struct example
#[derive(Debug)]
struct Color(u8, u8, u8); // RGB values

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }
    
    fn is_grayscale(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }
    
    fn brightness(&self) -> f32 {
        (self.0 as f32 + self.1 as f32 + self.2 as f32) / (3.0 * 255.0)
    }
}

// Unit struct example (no data)
struct UnitStruct;

fn exercise_4_structs() {
    println!("\n=== Exercise 4: Structs and Methods ===");
    
    // Creating instances
    let person1 = Person::new(
        String::from("Alice"),
        25,
        String::from("alice@example.com")
    );
    
    let mut person2 = Person::new_simple(String::from("Bob"), 17);
    
    println!("Person 1: {:?}", person1);
    println!("Person 2: {:?}", person2);
    
    // Using methods
    println!("{}", person1.introduce());
    println!("Is person1 an adult? {}", person1.is_adult());
    println!("Is person2 an adult? {}", person2.is_adult());
    
    // Mutable method
    person2.have_birthday();
    println!("Now is person2 an adult? {}", person2.is_adult());
    
    // Method that consumes the struct
    let summary = person1.into_summary();
    println!("Summary: {}", summary);
    // person1 can no longer be used
    
    // Tuple struct example
    let red = Color::new(255, 0, 0);
    let gray = Color::new(128, 128, 128);
    
    println!("Red color: {:?}", red);
    println!("Is red grayscale? {}", red.is_grayscale());
    println!("Red brightness: {:.2}", red.brightness());
    
    println!("Gray color: {:?}", gray);
    println!("Is gray grayscale? {}", gray.is_grayscale());
    println!("Gray brightness: {:.2}", gray.brightness());
    
    // Destructuring structs
    let Color(r, g, b) = red;
    println!("Red RGB values: r={}, g={}, b={}", r, g, b);
}

// ============================================================================
// EXERCISE 5: Enums and Pattern Matching
// Understanding: Sum types, Option, Result, match expressions
// ============================================================================

#[derive(Debug)]
enum Weather {
    Sunny,
    Rainy,
    Cloudy,
    Snowy { temperature: i32 },
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quitting application"),
            Message::Move { x, y } => println!("Moving to coordinates ({}, {})", x, y),
            Message::Write(text) => println!("Displaying message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn describe_weather(weather: &Weather) -> String {
    match weather {
        Weather::Sunny => String::from("It's a beautiful sunny day!"),
        Weather::Rainy => String::from("Don't forget your umbrella!"),
        Weather::Cloudy => String::from("The sky is overcast."),
        Weather::Snowy { temperature } => {
            if *temperature < 0 {
                String::from("It's snowing and freezing!")
            } else {
                String::from("It's snowing but not too cold.")
            }
        }
    }
}

// Working with Option<T>
fn find_item_index(items: &[&str], target: &str) -> Option<usize> {
    for (index, item) in items.iter().enumerate() {
        if *item == target {
            return Some(index);
        }
    }
    None
}

// Working with Result<T, E>
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn exercise_5_enums() {
    println!("\n=== Exercise 5: Enums and Pattern Matching ===");
    
    // Basic enum usage
    let today = Weather::Sunny;
    let tomorrow = Weather::Snowy { temperature: -5 };
    
    println!("Today: {}", describe_weather(&today));
    println!("Tomorrow: {}", describe_weather(&tomorrow));
    
    // Enum with different data types
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for message in &messages {
        message.process();
    }
    
    // Working with Option
    let fruits = ["apple", "banana", "orange", "grape"];
    
    match find_item_index(&fruits, "banana") {
        Some(index) => println!("Found banana at index {}", index),
        None => println!("Banana not found"),
    }
    
    match find_item_index(&fruits, "kiwi") {
        Some(index) => println!("Found kiwi at index {}", index),
        None => println!("Kiwi not found"),
    }
    
    // Alternative Option handling with if let
    if let Some(index) = find_item_index(&fruits, "orange") {
        println!("Orange is at index {}", index);
    }
    
    // Working with Result
    let calculations = vec![(10.0, 2.0), (5.0, 0.0), (15.0, 3.0)];
    
    for (a, b) in calculations {
        match divide(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(error) => println!("Error dividing {} by {}: {}", a, b, error),
        }
    }
    
    // Using unwrap_or for default values
    let safe_division = divide(10.0, 0.0).unwrap_or(0.0);
    println!("Safe division result: {}", safe_division);
}

// ============================================================================
// EXERCISE 6: Collections - Vectors
// Understanding: Dynamic arrays, growth, iteration
// ============================================================================

fn exercise_6_vectors() {
    println!("\n=== Exercise 6: Working with Vectors ===");
    
    // Creating vectors
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    
    let mut scores = vec![85, 92, 78, 96, 88];
    println!("Initial scores: {:?}", scores);
    
    // Accessing elements
    let first_score = &scores[0]; // Panics if index is out of bounds
    println!("First score: {}", first_score);
    
    // Safe access with get()
    match scores.get(10) {
        Some(score) => println!("Score at index 10: {}", score),
        None => println!("No score at index 10"),
    }
    
    // Modifying vectors
    scores.push(91);
    scores[1] = 95; // Modify existing element
    println!("After modifications: {:?}", scores);
    
    // Iterating over vectors
    println!("All scores:");
    for score in &scores {
        println!("  Score: {}", score);
    }
    
    // Iterating with indices
    for (index, score) in scores.iter().enumerate() {
        println!("  Score #{}: {}", index + 1, score);
    }
    
    // Mutable iteration
    for score in &mut scores {
        *score += 5; // Add 5 points to each score
    }
    println!("After bonus points: {:?}", scores);
    
    // Vector operations
    println!("Vector length: {}", scores.len());
    println!("Is empty? {}", scores.is_empty());
    
    // Remove elements
    let last_score = scores.pop(); // Returns Option<T>
    match last_score {
        Some(score) => println!("Removed last score: {}", score),
        None => println!("Vector was empty"),
    }
    
    scores.remove(0); // Remove at specific index
    println!("After removals: {:?}", scores);
    
    // Working with different types in vectors using enums
    #[derive(Debug)]
    enum Value {
        Integer(i32),
        Float(f64),
        Text(String),
    }
    
    let mixed_values = vec![
        Value::Integer(42),
        Value::Float(3.14),
        Value::Text(String::from("Hello")),
        Value::Integer(-7),
    ];
    
    for value in &mixed_values {
        match value {
            Value::Integer(i) => println!("Integer: {}", i),
            Value::Float(f) => println!("Float: {}", f),
            Value::Text(s) => println!("Text: {}", s),
        }
    }
}

// ============================================================================
// EXERCISE 7: Collections - Strings
// Understanding: String vs &str, UTF-8, manipulation
// ============================================================================

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    
    s // Return the whole string if no space found
}

fn count_vowels(s: &str) -> usize {
    let vowels = "aeiouAEIOU";
    s.chars().filter(|c| vowels.contains(*c)).count()
}

fn replace_spaces_with_underscores(s: &str) -> String {
    s.replace(' ', "_")
}

fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn exercise_7_strings() {
    println!("\n=== Exercise 7: Working with Strings ===");
    
    // String creation and manipulation
    let mut greeting = String::from("Hello");
    greeting.push_str(", World!");
    greeting.push('!');
    println!("Greeting: {}", greeting);
    
    // String vs &str
    let string_literal = "This is a string literal"; // &str
    let string_object = String::from("This is a String object");
    
    println!("Literal: {}", string_literal);
    println!("Object: {}", string_object);
    
    // String slicing
    let hello = &greeting[0..5];
    let world = &greeting[7..12];
    println!("Hello: '{}', World: '{}'", hello, world);
    
    // Working with our helper functions
    let sentence = "The quick brown fox jumps over the lazy dog";
    println!("Original: {}", sentence);
    println!("First word: '{}'", first_word(sentence));
    println!("Vowel count: {}", count_vowels(sentence));
    println!("With underscores: {}", replace_spaces_with_underscores(sentence));
    println!("Capitalized: {}", capitalize_words(sentence));
    
    // UTF-8 demonstration
    let emoji_string = "Hello 🦀 Rust 🚀 World!";
    println!("Emoji string: {}", emoji_string);
    println!("Length in bytes: {}", emoji_string.len());
    println!("Length in chars: {}", emoji_string.chars().count());
    
    // Iterating over characters
    println!("Characters:");
    for (i, c) in emoji_string.chars().enumerate() {
        println!("  {}: '{}'", i, c);
    }
    
    // String concatenation methods
    let part1 = String::from("Hello");
    let part2 = ", ";
    let part3 = "Rust!";
    
    // Method 1: Using +
    let combined1 = part1 + part2 + part3;
    println!("Combined with +: {}", combined1);
    
    // Method 2: Using format! macro
    let part1_again = String::from("Hello");
    let combined2 = format!("{}{}{}", part1_again, part2, part3);
    println!("Combined with format!: {}", combined2);
    
    // String formatting
    let name = "Alice";
    let age = 30;
    let formatted = format!("Name: {}, Age: {}, Next year: {}", name, age, age + 1);
    println!("Formatted: {}", formatted);
}

// ============================================================================
// EXERCISE 8: Collections - Hash Maps
// Understanding: Key-value storage, entry API, iteration
// ============================================================================

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        // Convert to lowercase for case-insensitive counting
        let word = word.to_lowercase();
        // Remove punctuation (simple approach)
        let word = word.trim_matches(|c: char| !c.is_alphabetic());
        
        if !word.is_empty() {
            // Using entry API - elegant way to update or insert
            let count = word_count.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }
    
    word_count
}

fn exercise_8_hashmaps() {
    println!("\n=== Exercise 8: Working with Hash Maps ===");
    
    // Creating and populating a HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    println!("Initial scores: {:?}", scores);
    
    // Accessing values
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team not found"),
    }
    
    // Using get with unwrap_or for default values
    let yellow_score = scores.get("Yellow").unwrap_or(&0);
    println!("Yellow team score (default): {}", yellow_score);
    
    // Iterating over HashMap
    println!("All team scores:");
    for (team, score) in &scores {
        println!("  {}: {}", team, score);
    }
    
    // Updating values
    scores.insert(String::from("Blue"), 25); // Overwrites old value
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(100); // Won't overwrite
    
    println!("After updates: {:?}", scores);
    
    // Updating based on old value
    let text = "hello world wonderful world";
    let mut word_counts = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word counts: {:?}", word_counts);
    
    // Using our word frequency function
    let sample_text = "The quick brown fox jumps over the lazy dog. The fox is quick and the dog is lazy.";
    let frequencies = word_frequency(sample_text);
    
    println!("Word frequencies in sample text:");
    // Sort by frequency (descending)
    let mut freq_vec: Vec<_> = frequencies.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (word, count) in freq_vec {
        println!("  '{}': {}", word, count);
    }
    
    // Creating HashMap from vectors
    let teams = vec![String::from("Alpha"), String::from("Beta")];
    let initial_scores = vec![100, 50];
    
    let team_scores: HashMap<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    
    println!("Team scores from vectors: {:?}", team_scores);
    
    // Demonstrating ownership with HashMap
    let key = String::from("Favorite Color");
    let value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(key, value);
    // key and value are no longer accessible here - moved into map
    
    // But we can still access them through the map
    if let Some(color) = map.get("Favorite Color") {
        println!("Favorite Color: {}", color);
    } else {
        println!("No favorite color found");
    }
}

// ============================================================================
// MAIN FUNCTION TO RUN ALL EXERCISES
// ============================================================================
fn main() {
    exercise_1_variables_and_types();
    exercise_2_functions_and_control();
    exercise_3_ownership();
    exercise_4_structs();
    exercise_5_enums();
    exercise_6_vectors();
    exercise_7_strings();
    exercise_8_hashmaps();
    
    println!("\nAll exercises completed successfully!");
}
// This code provides a comprehensive set of exercises covering Rust's core concepts,
// including variables, functions, ownership, structs, enums, collections, and more.
// Each exercise is designed to reinforce understanding of Rust's unique features and idioms.
// Feel free to modify and expand upon these exercises as you continue learning Rust!