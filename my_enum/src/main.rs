#[derive(Debug)]
enum Climate {
    Tropical,
    Dry,
    Temperate,
    Continential,
    Polar,
}

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    climate: Climate,
}

fn main() {
    let kalmykia = Country {
        population: 500_000,
        capital: String::from("Elista"),
        leader_name: String::from("Batu Khasikov"),
        climate: Climate::Continential,
    };

    println!("The climate is {:#?}", kalmykia);
}
