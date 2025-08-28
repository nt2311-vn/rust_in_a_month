fn main() {
    let my_number = 5;
    if my_number % 2 == 1 && my_number > 0 {
        println!("It's a positive odd number");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }

    let other_number: u8 = 5;

    match other_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("it's some other number"),
    }

    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };

    println!("Second number {second_number}");
}
