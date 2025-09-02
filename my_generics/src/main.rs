use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn return_item<T>(item: T) -> T {
    println!("Here is your item.");
    item
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input1: U, input2: U) {
    println!(
        "{statement}! Is {input1} greater than {input2}? {}",
        input1 > input2
    );
}

fn main() {
    let item = return_item(5);
    println!("Item {item}");
    print_item(5);

    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };

    let number = 55;
    print_item(number);
    print_item(charlie);

    compare_and_display("Listen up!", 9, 8);
}
