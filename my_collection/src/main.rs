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

    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(&1));

    let mut old_hashmap_values = Vec::new();
    let hashmap_entries = [
        (1, "L'Allemagne Moderne"),
        (1, "Le Petit Prince"),
        (1, "섀도우 오브 유어 스마일"),
        (1, "Eye of the World"),
    ];

    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap.insert(key, value) {
            println!("Overwriting {old_value} with {value}!");
            old_hashmap_values.push(old_value);
        }
    }

    println!("All old values: {old_hashmap_values:?}");

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];

    let mut newbook_hashmap = HashMap::new();

    for book in &book_collection {
        newbook_hashmap.entry(book).or_insert(true);
    }

    for (book, true_or_false) in newbook_hashmap {
        println!("Do we have {book}? {true_or_false}");
    }

    let mut book_quantity = HashMap::new();

    for book in &book_collection {
        let return_value = book_quantity.entry(book).or_insert(0);
        *return_value += 1;
    }

    for (book, number) in book_quantity {
        println!("{book}, {number}");
    }

    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    for (male_or_female, numbers) in survey_hash {
        println!("{male_or_female}: {numbers:?}");
    }
}
