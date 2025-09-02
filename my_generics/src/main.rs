fn return_item<T>(item: T) -> T {
    println!("Here is your item.");
    item
}

fn main() {
    let item = return_item(5);
    println!("Item {item}");
}
