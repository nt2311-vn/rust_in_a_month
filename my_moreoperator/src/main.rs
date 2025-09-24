fn main() {
    let num_array = ["8", "9", "Hi", "9898989898"];

    let mut char_vec = vec![];

    for index in 0..5 {
        char_vec.push(
            num_array
                .get(index)
                .and_then(|number| number.parse::<u32>().ok())
                .and_then(|number| char::try_from(number).ok()),
        );
    }

    println!("{:?}", char_vec)
}
