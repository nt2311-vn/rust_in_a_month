#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
}

fn works() -> &'static str {
    "I live forever!"
}

fn returns_str() -> &'static str {
    "I am a str"
}

fn main() {
    let my_str = returns_str();
    println!("{my_str}");

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}
