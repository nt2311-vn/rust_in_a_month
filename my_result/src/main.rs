fn check_error() -> Result<(), ()> {
    Ok(())
}

fn see_if_number_is_even(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(());
    } else {
        return Err(());
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err(format!("Sorry, bad number. Expected: 5 Got: {number}")),
    }
}

fn main() {
    let _ = check_error();

    if see_if_number_is_even(5).is_ok() {
        println!("It's okay, guys");
    } else {
        println!("It's an error, guys");
    }

    for number in 4..=7 {
        println!("{:?}", check_if_five(number))
    }
}
