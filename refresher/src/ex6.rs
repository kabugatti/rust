// Practice working with String vs &str and string slices
fn main() {
    let sentence = String::from("Hello, Rust programming world!");
    
    // Write functions that:
    // 1. Return the first word of a sentence
    // 2. Count vowels in a string
    // 3. Replace all spaces with underscores
    
    fn first_word(s: &str) -> &str {
        // Your implementation here
        // Hint: find the first space and return a slice
        // Logic should be beginner friendly
        let mut ans = String::new();

        for c in s.chars() {
            if c == ' ' {
                break;
            }
            ans.push(c);
        }
        
        ans.as_str();
    }
    
    fn count_vowels(s: &str) -> usize {
        // Your implementation here
        // Hint: iterate through chars and count a, e, i, o, u
        let mut vowel_count: usize = 0;
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        for c in s.chars() {
            if vowels.contains(&c) {
                vowel_count += 1;
            }
        }

        vowel_count
    }
    
    fn replace_spaces(s: &str) -> String {
        // Your implementation here
        // Hint: use the replace method
        s.replace(' ', "_")
    }

    let r1 = first_word(&sentence);
    let r2 = count_vowels(&sentence);
    let r3 = replace_spaces(&sentence);

    println!("First word: {}", r1);
    println!("Vowel count: {}", r2);
    println!("Replaced spaces: {}", r3);
}