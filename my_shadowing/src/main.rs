fn main() {
    let my_number = 8;
    println!("My number {}", my_number);

    {
        let my_number = 9.2;
        println!("My number {}", my_number);
    }

    println!("{}", my_number);
}
