struct File(String);

fn main() {
    let my_file = File(String::from("I am a file contents"));
    let my_string = String::from("I am a file contents");

    println!("{}", my_file.0 == my_string);
}
