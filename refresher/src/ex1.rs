// Exercise 1: Ownership and Borrowing Review

// Fix this code to compile - focus on ownership rules
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1; // s1 is moved here
    
    // This won't compile - why?
    println!("{}", s1);
    println!("{}", s2);
    
    // How would you fix it to use both s1 and s2?
}