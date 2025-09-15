use std::num::ParseIntError;

fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}
fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];

    for item in str_vec {
        let parsed = parse_and_log_str(item);
        println!("Result: {parsed:?}");
    }
}
