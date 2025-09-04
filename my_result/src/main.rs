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

fn main() {
    let _ = check_error();

    if see_if_number_is_even(5).is_ok() {
        println!("It's okay, guys");
    } else {
        println!("It's an error, guys");
    }
}
