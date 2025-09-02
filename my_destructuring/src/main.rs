#[derive(Debug)]
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

fn check_if_happy(person: &Person) {
    println!("Is {} happy? {}", person.name, person.happiness);
}

fn check_if_happy_destructed(
    Person {
        name, happiness, ..
    }: &Person,
) {
    println!("Is {name} happy? {happiness}");
}

#[derive(Debug)]
struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: &str, name_before: &str, population: u32, date_founded: u32) -> Self {
        Self {
            name: String::from(name),
            name_before: String::from(name_before),
            population,
            date_founded,
        }
    }

    fn print_names(&self) {
        let City {
            name, name_before, ..
        } = self;

        println!("The city {name} used to be called {name_before}");
    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let Person {
        ref name,
        ref real_name,
        height,
        happiness,
    } = papa_doc;
    println!("They call him {name} but his real name is {real_name}. He is {height} cm tall and is he happy? {happiness}");

    let tallin = City::new("Tallinn", "Reval", 426_538, 1219);
    tallin.print_names();

    check_if_happy(&papa_doc);
    check_if_happy_destructed(&papa_doc);
}
