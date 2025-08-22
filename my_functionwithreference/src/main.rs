fn print_country(country_name: &str) {
    println!("{country_name}");
}

fn main() {
    let country = String::from("Austria");
    print_country(&country);
    print_country(&country);
}
