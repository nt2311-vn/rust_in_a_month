fn main() {
    let array_vec = Vec::from([8, 9, 10]);
    println!("Vec from array: {array_vec:?}");

    let str_vec = Vec::from("What kind of Vec am I");
    println!("Vec from str: {str_vec:?}");

    let string_vec = Vec::from("What will a String be?".to_string());
    println!("Vec from string: {string_vec:?}");
}
