fn works() -> &'static str {
    "I live forever!"
}

fn returns_str() -> &'static str {
    "I am a str"
}

fn main() {
    let my_str = returns_str();
    println!("{my_str}");
}
