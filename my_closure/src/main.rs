fn main() {
    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5 + 5);

    let other_closure = || {
        let number = 7;
        let other_number = 10;

        println!("The two numbers are {number} and {other_number}.");
    };

    other_closure();
    let number_one = 6;
    let number_two = 10;
    let taken_external_closure = |x: i32| println!("{}", number_one + number_two + x);
    taken_external_closure(5);
}
