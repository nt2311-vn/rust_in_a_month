fn main() {
    let mut new_vec = Vec::new();
    let mut counter = 1;

    loop {
        new_vec.push(counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    println!("{new_vec:?}");

    let functional_vec = (1..).take(9).collect::<Vec<i32>>();
    println!("{functional_vec:?}");

    let fixed_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let collect_vec = fixed_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    println!("{collect_vec:?}");
}
