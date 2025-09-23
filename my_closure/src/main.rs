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
}
