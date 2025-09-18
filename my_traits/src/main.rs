struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

struct Animal {
    name: String,
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
}
