fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let country = String::from("Kiribati");
    prints_country(country.clone());
    prints_country(country);
}
