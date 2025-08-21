fn times_two(number: i32) -> i32 {
    number * 2
}
fn main() {
    let my_number = 8;
    println!("My number {}", my_number);

    {
        let my_number = 9.2;
        println!("My number {}", my_number);
    }

    println!("{}", my_number);

    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y;
        x
    };

    println!("The number is now: {}", final_number);

    let x = 9;
    let mut x = x as f32;
    x += 0.5;

    println!("The number is now: {}", x);
}
