fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref());
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
