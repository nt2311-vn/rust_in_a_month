use std::fmt::Debug;

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
}
