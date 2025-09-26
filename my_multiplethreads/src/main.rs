use std::cell::{Cell, RefCell};

enum MapDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn give_direction(direction: &MapDirection) {
    use MapDirection::*;
    match direction {
        North => println!("You are heading north."),
        NorthEast => println!("You are heading northeast."),
        East => println!("You are heading east."),
        SouthEast => println!("You are heading south east"),
        South => println!("You are heading south"),
        SouthWest => println!("You are heading south west"),
        West => println!("You are heading west"),
        NorthWest => println!("You are heading north west"),
    }
}
fn main() {}
