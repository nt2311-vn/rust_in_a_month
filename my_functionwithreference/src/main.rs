fn print_country(country_name: &str) {
    println!("{country_name}");
}

fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it says: {country_name}");
}

fn adds_hungary(mut string_to_add_hungary_to: String) {
    string_to_add_hungary_to.push_str("-Hungary");
    println!("{}", string_to_add_hungary_to);
}

fn main() {
    let country = String::from("Austria");
    print_country(&country);
    print_country(&country);

    let mut new_country = String::from("Austria");
    add_hungary(&mut new_country);

    adds_hungary(country);
}
