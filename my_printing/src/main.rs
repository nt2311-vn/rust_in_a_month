fn give_number() -> i32 {
    8
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}
fn main() {
    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);

    let my_number = 8;
    println!("Hello, number {my_number}");

    let color1 = "red";
    let color2 = "green";
    let color3 = "blue";
    println!("I like {color1} and {color2} and {color3}");

    let naver_base_url = "naver";
    let google_base_url = "google";
    let microsoft_base_url = "microsoft";

    println!("The url is www.{}.com", naver_base_url);
    println!("The url is www.{}.com", google_base_url);
    println!("The url is www.{}.com", microsoft_base_url);

    let combined_number = {
        let second_number = 8;
        second_number + 9
    };

    println!("My number is: {:?}", combined_number);

    let doesnt_print = ();
    println!("This will not print: {:?}", doesnt_print);

    print!("\t Start with a tab\nand move to a new line\n");
    println!(
        "Inside quotes
you can write over
many lines
and it will print just fine."
    );
    println!("Here are two escape characters: \\n and \\t");
    println!(
        r#"He said, "You can find the file at
c:\\files\\my_documents\\file.txt.\" Then I found the file."#
    );

    let my_string = "' Ice to see you,' he said.";
    let quote_string = r#""Ice to see you," he said."#;
    let hashtag_string = r##"The hashtag "#IceToSeeYou" had become
very popular."##;
    let many_hashtag = r####""You don't have to type "###" to
use a hashtag. You can just use #."####;

    println!(
        "{}\n{}\n{}\n{}\n",
        my_string, quote_string, hashtag_string, many_hashtag
    );

    println!("{:?}", b"This will look like numbers");
    println!("{:?}", br##"I like to write "#"."##);

    println!("{:X}", '행' as u32);
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    let other_number = 555;
    println!(
        "Binary: {:b}, Hexadecimal: {:x}, octal: {:o}",
        other_number, other_number, other_number
    );

    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!(
        "This is {1} {2}, son of {0} {2}",
        father_name, son_name, family_name
    );

    println!("{city1}, is in {country} and {city2} is also in {country}, but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );

    let letter = "a";
    println!("{:ㅎ^11}", letter);
}
