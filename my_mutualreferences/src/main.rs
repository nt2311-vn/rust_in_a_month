fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;

    *num_ref += 10;
    println!("{}", my_number);

    let second_number = 800;
    let tripple_reference = &&&second_number;
    println!("Are they equal? {}", second_number == ***tripple_reference);

    let mut new_number = 10;

    let number_change = &mut new_number;
    *number_change += 10;
    let number_ref = &new_number;

    println!("{}", number_ref);
}
