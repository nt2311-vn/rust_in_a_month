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

    let mut other_num_vec = Vec::with_capacity(8);
    other_num_vec.push('a');
    println!("{}", other_num_vec.capacity());
    other_num_vec.push('a');
    println!("{}", other_num_vec.capacity());
    other_num_vec.push('a');
    println!("{}", other_num_vec.capacity());
    other_num_vec.push('a');
    other_num_vec.push('a');
    println!("{}", other_num_vec.capacity());

    let into_vec: Vec<u8> = [1, 2, 3].into();
    let into_vec2: Vec<_> = [9, 0, 10].into();

    println!("Into vec: {:?}, into_vec2: {:?}", into_vec, into_vec2);
}
