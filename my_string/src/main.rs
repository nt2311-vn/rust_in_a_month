fn main() {
    let _name = "자우림";
    let _other_name = String::from("Adrian Fahrenheit Țepeș");

    let name = "Ⴡ";
    println!("My name is actually {}", name);

    let size_of_string = std::mem::size_of::<String>();
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_f64 = std::mem::size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");
    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");

    println!("A String is Sized and always {size_of_string} bytes.");
    println!("An i8 is Sized and always {size_of_i8} bytes.");
    println!("A f64 is Sized and alays {size_of_f64} bytes.");
    println!("But a &str is not Sized: '자우림' is {size_of_jaurim} bytes.");
    println!("And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes - not Sized.");

    let my_name = "Billybrobby";
    let country = "USA";
    let home = "Korea";
    let together = format!("I am {my_name} from {country} but I live in {home}.");

    println!("{together}");

    let my_string: String = "Try to make this a String".into();
    println!("{my_string}");
}
