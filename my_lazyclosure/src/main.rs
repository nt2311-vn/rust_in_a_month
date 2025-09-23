fn main() {
    let num_vec = vec![2, 4, 6];

    let double_vec: Vec<i32> = num_vec.iter().map(|num| num * 2).collect();
    println!("{:?}", double_vec);

    num_vec
        .iter()
        .enumerate()
        .map(|(index, num)| format!("Index {index} is {num}"));
}
