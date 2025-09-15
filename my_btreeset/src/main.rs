use std::collections::BTreeSet;

fn main() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];

    let mut current_number = i32::MIN;
    let mut number_set = BTreeSet::new();

    for number in many_numbers {
        number_set.insert(number);
    }

    for number in number_set {
        if number < current_number {
            println!("This will never happen again");
        }

        current_number = number;
    }
}
