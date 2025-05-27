// Exercise 4: Collections and Iterators

// Write a function that takes a vector of integers
// and returns a new vector with only the even numbers, doubled
fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // Your implementation here
    // Try using iterator methods like .filter() and .map()
    numbers.into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n * 2)
        .collect()
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = process_numbers(nums);
    println!("{:?}", result); // Should print [4, 8, 12, 16, 20]
}