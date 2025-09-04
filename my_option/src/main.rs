fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_options(my_option: &Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {number}!"),
            None => println!("Found a None!"),
        }
    }
}

fn main() {
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    // println!(
    //     "{:?}, {:?}",
    //     try_take_fifth(small).unwrap(),
    //     try_take_fifth(big).unwrap()
    // );

    // let mut option_vec = Vec::new();
    // option_vec.push(try_take_fifth(small));
    // option_vec.push(try_take_fifth(big));
    // handle_options(&option_vec);

    for vec in vec![small, big] {
        let inside_number = try_take_fifth(vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap());
        } else {
            println!("We got nothing.");
        }
    }
}
