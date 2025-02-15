use std::collections::BTreeSet;

fn main() {
    let s1 = "elevenplustwo";
    let s2 = "twelveplusone";

    if s1.len() != s2.len() {
        println!("Not anagrams!");
        return
    }

    let mut s1_bt = BTreeSet::<char>::new();
    let mut s2_bt = BTreeSet::<char>::new();

    for c in s1.chars() {
        s1_bt.insert(c);
    }

    for c in s2.chars() {
        s2_bt.insert(c);
    }

    if s1_bt == s2_bt {
        println!("{} and {} are anagrams!", s1, s2);
    } else {
        println!("Not anagrams!");
    }
}
