use std::collections::HashMap;

fn main() {
    let num_vec = vec![2, 4, 6];

    let double_vec: Vec<i32> = num_vec.iter().map(|num| num * 2).collect();
    println!("{:?}", double_vec);

    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];

    let number_word_hasmap = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect::<HashMap<_, _>>();

    println!(
        "The value at key 2 is: {}",
        number_word_hasmap.get(&2).unwrap()
    );

    let numbers_together = "140399923481800622623218009598281";

    for (index, num) in numbers_together.char_indices() {
        match (index % 3, num) {
            (0 | 1, num) => print!("{num}"),
            _ => print!("{num}\t"),
        }
    }
}
