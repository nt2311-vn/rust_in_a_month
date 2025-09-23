fn main() {
    let vector1 = vec![1, 2, 3];

    let mut vector2 = vec![10, 20, 30];

    for num in vector1.iter() {
        println!("Printing a &i32: {num}");
    }

    for num in vector1 {
        println!("Printing an i32: {num}");
    }

    for num in vector2.iter_mut() {
        *num *= 10;
        println!("num is now {num}");
    }

    println!("{vector2:?}");
}
