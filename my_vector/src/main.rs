use std::vec;

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);

    println!("My vec: {:?}", my_vec);
    let other_vec = vec![8, 10, 10];
    println!("Other vec: {:?}", other_vec);

    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_of_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!(
        "Three of five: {:?}, 
start at two: {:?}
end at five: {:?}
everything: {:?}",
        three_of_five, start_at_two, end_at_five, everything
    );

    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity());

    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');

    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
}
