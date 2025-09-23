fn main() {
    (1..=3).for_each(|num| println!("{num}"));
    (1..=3).for_each(|num| {
        println!("Got a {num}!");

        if num % 2 == 0 {
            println!("It's even")
        } else {
            println!("It's odd")
        };
    });
}
