#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has population of {:?}.", city.name, city.population);
        }
    }
}

fn main() {
    let array_vec = Vec::from([8, 9, 10]);
    println!("Vec from array: {array_vec:?}");

    let str_vec = Vec::from("What kind of Vec am I");
    println!("Vec from str: {str_vec:?}");

    let string_vec = Vec::from("What will a String be?".to_string());
    println!("Vec from string: {string_vec:?}");

    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let findland = Country::from(finland_cities);
    findland.print_cities();
}
