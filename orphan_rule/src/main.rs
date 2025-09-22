#[derive(Clone, Debug)]
struct File(String);

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_bytes = format!("{:?}", self.0.as_bytes());
        write!(f, "{as_bytes}")
    }
}

fn main() {
    let my_file = File(String::from("I am a file contents"));
    let my_string = String::from("I am a file contents");

    let file = File(String::from("I am file contents"));
    println!("{}", my_file.0 == my_string);
    println!("{file:?}");
    println!("{file}");
}
