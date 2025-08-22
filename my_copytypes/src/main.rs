fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn get_length(input: String) {
    println!("It's {} words long", input.split_whitespace().count());
}

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let country = String::from("Kiribati");
    prints_country(country.clone());
    prints_country(country);

    let mut my_string = String::new();

    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length(my_string.clone());
    }
}
