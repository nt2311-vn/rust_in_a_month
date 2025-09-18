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

impl DogLike for Dog {}
impl DogLike for Parrot {}

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
}
