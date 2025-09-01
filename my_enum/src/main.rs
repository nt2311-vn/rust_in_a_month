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

#[derive(Debug)]
enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;

    let happiness_level = match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };

    happiness_level
}

#[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
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

    let my_mood = Mood::Happy;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {happiness_level}");

    use Season::*;

    let four_season = vec![Spring, Summer, Autumn, Winter];

    for season in four_season {
        println!("{}", season as u32);
    }
}
