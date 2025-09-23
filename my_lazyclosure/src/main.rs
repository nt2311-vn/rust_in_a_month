fn main() {
    let num_vec = vec![2, 4, 6];

    let double_vec: Vec<i32> = num_vec.iter().map(|num| num * 2).collect();
    println!("{:?}", double_vec);
}
