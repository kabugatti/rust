use std::collections::HashSet;

fn main() {
    let list1 = vec![1, 2, 3, 4, 5];
    let list2 = vec![3, 4, 5, 6, 7];

    let mut first_hs = HashSet::<u8>::new();
    let mut intersection = HashSet::<u8>::new();

    for i in list1.into_iter() {
        first_hs.insert(i);
    }

    for i in list2.into_iter() {
        if let Some(_val) = first_hs.get(&i) {
            intersection.insert(i);
        }
    }

    println!("Intersection: {:?}", intersection);
}
