use std::fmt::Debug;

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
}
