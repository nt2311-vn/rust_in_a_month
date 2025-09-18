struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof woof!");
    }

    fn run(&self) {
        println!("The dog is running!");
    }
}

impl DogLike for Dog {
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

fn main() {
    let rover = Dog {
        name: "Rover".to_string(),
    };

    let brian = Parrot {
        name: "Brian".to_string(),
    };

    rover.bark();
    rover.run();
    brian.bark();
    brian.run();
}
