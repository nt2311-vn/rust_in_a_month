use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug)]
struct City {
    name: String,
    population: HashMap<i32, i32>,
}

#[derive(Debug)]
struct CityB {
    name: String,
    population: BTreeMap<i32, i32>,
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    for (year, population) in tallinn.population {
        println!("In {year}, Tallinn had a population of {population}");
    }

    let _tallinb = CityB {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "German");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));
}
