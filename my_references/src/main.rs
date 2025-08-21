fn main() {
    let my_number = 15;
    let single_reference = &my_number;
    let _double_reference = &single_reference;
    let _five_references = &&&&&my_number;

    let country = String::from("Australia");
    let ref_one = &country;
    let _ref_two = &country;

    println!("{}", ref_one);
}
