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

    let mut tallinb = CityB {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };
}
