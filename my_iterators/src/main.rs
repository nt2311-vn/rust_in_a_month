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

    let another_vector1 = vec![1, 2, 3];

    let another_vector1_a = another_vector1.iter().map(|x| x * 10).collect::<Vec<i32>>();

    let another_vector1_b = another_vector1
        .into_iter()
        .map(|x| x * 10)
        .collect::<Vec<i32>>();

    let mut another_vector2 = vec![10, 20, 30];
    another_vector2.iter_mut().for_each(|x| *x += 100);

    println!("{:?}", another_vector1_a);
    println!("{:?}", another_vector1_b);
    println!("{:?}", another_vector2);
}
