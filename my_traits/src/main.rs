use std::fmt;

struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

struct Animal {
    name: String,
}

struct Cat {
    name: String,
    age: u8,
}

trait DogLike {
    fn bark(&self) {}

    fn run(&self) {}
}

impl DogLike for Dog {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("{} the dog is running!", self.name);
    }
}
impl DogLike for Parrot {
    fn bark(&self) {
        println!("Kek kek!");
    }

    fn run(&self) {
        println!("{} the parrot is running!", self.name);
    }
}

impl DogLike for Animal {
    fn bark(&self) {
        println!("{}, stop barking!", self.name);
    }

    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}

fn print_excitedly(input: String) {
    println!("{input}!!!!!");
}

fn main() {
    let rover = Dog {
        name: "Rover".to_string(),
    };

    let brian = Parrot {
        name: "Brian".to_string(),
    };

    let ibrahim = Animal {
        name: "Ibrahim".to_string(),
    };

    rover.bark();
    rover.run();
    brian.bark();
    brian.run();
    ibrahim.run();
    ibrahim.bark();

    let mr_bup = Cat {
        name: "Bụp".to_string(),
        age: 3,
    };

    print_excitedly(mr_bup.to_string());
    println!(
        "Mr.Bụp's String is {} letters long.",
        mr_bup.to_string().chars().count()
    );
}
