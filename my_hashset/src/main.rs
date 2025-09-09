use std::{collections::HashSet, hash::Hash};

fn main() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];

    println!("How many numbers in the Vec? {}", many_numbers.len());

    let mut number_hashset = HashSet::new();

    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();

    println!(
        "There are {hashset_length} unique numbers. so we are missing {}.",
        50 - hashset_length
    );

    println!("It does not contain: ");
    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            print!("{number} ");
        }
    }
}
