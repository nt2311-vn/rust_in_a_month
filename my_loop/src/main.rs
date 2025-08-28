fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {counter}");
        if counter == 5 {
            break;
        }
    }

    let mut other_counter = 0;
    let mut other_counter2 = 0;

    println!("Now entering the first loop.");

    'first_loop: loop {
        other_counter += 1;
        println!("The other counter is now: {}", other_counter);

        if other_counter > 5 {
            println!("Now entering the second loop.");

            'second_loop: loop {
                println!("The second counter is now: {}", other_counter2);
                other_counter2 += 1;

                if other_counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}
