fn print_all_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must always have three items.");
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}
fn main() {
    let my_vec = vec![8, 9, 10];
    print_all_three_things(my_vec);

    let my_name = "Loki Laufeysons";

    assert!(
        my_name == "Loki Laufeyson",
        "Name {my_name} is wrong: should be Loki Laufeyson"
    );

    assert_eq!(
        my_name, "Loki Laufeyson",
        "{my_name} and Loki Laufeyson should be equal"
    );

    assert_ne!(
        my_name, "Mithridates",
        "You entered {my_name}. Input must not equal Mithridates"
    );
}
