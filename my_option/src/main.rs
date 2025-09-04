fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}
fn main() {
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}", try_take_fifth(small), try_take_fifth(big));
}
