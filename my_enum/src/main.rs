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

#[derive(Debug)]
enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

#[derive(Debug)]
enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };

    number
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

    use Star::*;

    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];

    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star!"),
            size if size >= 80 && size <= 200 => println!("This is a good-sized star."),
            other_size => println!("That star is pretty big! It's {other_size}"),
        }
    }

    let my_vec = vec![get_number(-800), get_number(8)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("A u32 with the value {number}"),
            Number::I32(number) => println!("An i32 with the value {number}"),
        }
    }
}
