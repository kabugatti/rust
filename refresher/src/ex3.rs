// Exercise 3: Error Handling with Result

// Write a function that parses a string to an integer
// and returns a custom error message if it fails
fn parse_number(s: &str) -> Result<i32, String> {
    // Your implementation here
    // Use s.parse::<i32>() and handle the error case
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Error: Failed to parse '{}' as an integer", s)),
    }
}

fn main() {
    let test_cases = vec!["42", "hello", "123", ""];
    
    for case in test_cases {
        match parse_number(case) {
            Ok(num) => println!("Parsed: {}", num),
            Err(msg) => println!("Error: {}", msg),
        }
    }
}