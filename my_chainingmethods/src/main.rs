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
}
