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

#[derive(Debug)]
enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the starts!"),
    }
}

fn main() {
    let kalmykia = Country {
        population: 500_000,
        capital: String::from("Elista"),
        leader_name: String::from("Batu Khasikov"),
        climate: Climate::Continential,
    };

    println!("The climate is {:#?}", kalmykia);

    let time = 8;
    let skytate = create_skystate(time);
    check_skystate(&skytate);
}
