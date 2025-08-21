fn give_number() -> i32 {
    8
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}
fn main() {
    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);

    let my_number = 8;
    println!("Hello, number {my_number}");

    let color1 = "red";
    let color2 = "green";
    let color3 = "blue";
    println!("I like {color1} and {color2} and {color3}");
}
