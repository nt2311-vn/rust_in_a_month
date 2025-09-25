#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    date_founded: u32,
}

struct Adventure<'a> {
    name: &'a str,
    hit_point: u32,
}

impl Adventure<'_> {
    fn take_damage(&mut self) {
        self.hit_point -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_point);
    }
}

impl std::fmt::Display for Adventure<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points.", self.name, self.hit_point)
    }
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

    let mut billy = Adventure {
        name: "Billy",
        hit_point: 100_000,
    };

    println!("{}", billy);
    billy.take_damage();
}
