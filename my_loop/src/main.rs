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

    let mut another_counter = 0;

    while another_counter < 5 {
        another_counter += 1;
        println!("The another counter is now: {another_counter}");
    }

    for number in 0..3 {
        println!("The number is: {}", number);
    }

    for number in 0..=3 {
        println!("The next number is: {}", number);
    }

    for _ in 0..3 {
        println!("Printing the same thing three times");
    }

    let mut next_counter = 5;

    let my_number = loop {
        next_counter += 1;
        if next_counter % 53 == 3 {
            break next_counter;
        }
    };

    print!("Next counter: {my_number}");
}
