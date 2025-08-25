fn main() {
    let array1 = ["One", "Two"];
    let array2 = ["One", "Two", "Five"];
    let my_array = ["a"; 5];
    println!("{:?}", my_array);

    println!("{:?}", b"Hello there");

    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]);

    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!(
        "Two to five: {two_to_five:?},
Start at one: {start_at_one:?},
End at five : {end_at_five:?},
Everything: {everything:?}
"
    );
}
